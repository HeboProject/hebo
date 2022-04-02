// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use super::pwd::Password;
use crate::error::{Error, ErrorKind};

/// FileAuth represents records in password_file.
#[derive(Debug)]
pub struct FileAuth(BTreeMap<String, Password>);

impl FileAuth {
    /// Parse password_file.
    pub fn new<P: AsRef<Path>>(password_file: P) -> Result<Self, Error> {
        let fd = File::open(password_file.as_ref())?;
        let reader = BufReader::new(fd);
        let mut map = BTreeMap::new();
        for line in reader.lines() {
            let line = line?;
            match Password::parse(&line) {
                Err(err) => {
                    log::error!("err: {:?}, line: {}", err, line);
                }
                Ok(None) => {
                    // continue
                }
                Ok(Some((username, password))) => {
                    map.insert(username.to_string(), password);
                }
            }
        }

        Ok(Self(map))
    }

    /// Check if (username, password) pair exists in records.
    pub fn is_match(&self, username: &str, password: &[u8]) -> Result<bool, Error> {
        match self.0.get(username) {
            None => Ok(false),
            Some(p) => p.is_match(password),
        }
    }
}

pub fn update_file_hash<P: AsRef<Path>>(password_file: P) -> Result<(), Error> {
    let fd = File::open(password_file.as_ref())?;
    let reader = BufReader::new(fd);
    let mut result = String::new();
    for line in reader.lines() {
        let line = line?;
        match Password::parse_raw_text(&line) {
            Err(err) => {
                log::error!("err: {:?}, line: {}", err, line);
            }
            Ok(None) => {
                // continue
            }
            Ok(Some((username, password))) => {
                let hashed_line = password.dump(username);
                result.push_str(&hashed_line);
                result.push_str("\n");
            }
        }
    }

    let mut fd = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(password_file.as_ref())?;
    fd.write(result.as_bytes()).map(drop).map_err(Into::into)
}

pub fn add_delete_users<P: AsRef<Path>>(
    password_file: P,
    add_users: &[&str],
    delete_users: &[&str],
) -> Result<(), Error> {
    let fd = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(password_file.as_ref())?;
    let reader = BufReader::new(fd);
    let mut users = BTreeMap::new();
    for line in reader.lines() {
        let line = line?;
        match Password::parse(&line) {
            Err(err) => {
                log::error!("Failed to parse line {:?}, got err: {:?}", line, err);
                return Err(err);
            }
            Ok(None) => {
                // continue
            }
            Ok(Some((username, password))) => {
                users.insert(username.to_string(), password);
            }
        }
    }

    // Add/update users
    for item in add_users {
        match Password::parse_raw_text(item) {
            Err(err) => {
                log::error!("Failed to parse pair {:?}, got err: {:?}", item, err);
                return Err(err);
            }
            Ok(None) => {
                log::info!("Ignore empty line: {}", item);
                // continue
            }
            Ok(Some((username, password))) => {
                users.insert(username.to_string(), password);
            }
        }
    }

    // Delete users
    for username in delete_users {
        if username.contains(':') {
            return Err(Error::from_string(
                ErrorKind::ParameterError,
                format!("Invalid username to delete: {:?}", username),
            ));
        }

        users.remove(*username);
    }

    let mut fd = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(password_file.as_ref())?;
    for (username, password) in users {
        let line = password.dump(&username);
        log::info!("line: {}", line);
        fd.write(line.as_bytes())?;
        fd.write(b"\n")?;
    }

    Ok(())
}
