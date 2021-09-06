// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::time::Duration;

use crate::error::Error;

/// Configuration for connection to pgsql server.
#[derive(Debug, Deserialize, Clone)]
pub struct PgSQLConnConfig {
    /// Use unix domain socket connection to PgSQL.
    ///
    /// Default is false.
    #[serde(default = "PgSQLConnConfig::default_use_uds")]
    pub use_uds: bool,

    /// Socket address to server.
    ///
    /// Default is None.
    #[serde(default = "PgSQLConnConfig::default_socket")]
    pub socket: Option<String>,

    /// PgSQL server ip or hostname.
    ///
    /// Default is "127.0.0.1"
    #[serde(default = "PgSQLConnConfig::default_ip")]
    pub ip: String,

    /// Server port number.
    ///
    /// Default is 5432.
    #[serde(default = "PgSQLConnConfig::default_port")]
    pub port: u16,

    /// PgSQL database number.
    ///
    /// Default is `hebo-mqtt`.
    #[serde(default = "PgSQLConnConfig::default_database")]
    pub database: String,

    /// Connection username.
    ///
    /// Default is `postgres`.
    #[serde(default = "PgSQLConnConfig::default_username")]
    pub username: String,

    /// Connection password.
    ///
    /// Default is empty.
    #[serde(default = "PgSQLConnConfig::default_password")]
    pub password: String,

    /// Connection pool.
    ///
    /// Default is 4.
    #[serde(default = "PgSQLConnConfig::default_pool_size")]
    pub pool_size: usize,

    /// Connection/query timeout in seconds.
    ///
    /// Default is 5s.
    #[serde(default = "PgSQLConnConfig::default_query_timeout")]
    pub query_timeout: u32,
}

impl PgSQLConnConfig {
    const fn default_use_uds() -> bool {
        false
    }

    fn default_socket() -> Option<String> {
        None
    }

    fn default_ip() -> String {
        "127.0.0.1".to_string()
    }

    const fn default_port() -> u16 {
        5432
    }

    fn default_username() -> String {
        "postgres".to_string()
    }

    fn default_password() -> String {
        String::new()
    }

    fn default_database() -> String {
        "hebo-mqtt".to_string()
    }

    const fn default_pool_size() -> usize {
        4
    }

    const fn default_query_timeout() -> u32 {
        5
    }
}

impl Default for PgSQLConnConfig {
    fn default() -> Self {
        Self {
            use_uds: Self::default_use_uds(),
            socket: Self::default_socket(),
            ip: Self::default_ip(),
            port: Self::default_port(),
            database: Self::default_database(),
            username: Self::default_username(),
            password: Self::default_password(),
            pool_size: Self::default_pool_size(),
            query_timeout: Self::default_query_timeout(),
        }
    }
}

impl PgSQLConnConfig {
    pub fn query_timeout(&self) -> Duration {
        Duration::from_secs(self.query_timeout as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgsql_config() {
        let config: Result<PgSQLConnConfig, Error> = toml::from_str(
            r#"
        use_ds = false
        database = "hebo-mqtt"
        username = "user1"
        password = "password1"
        pool_size = 8
        query_timeout = 6
        "#,
        )
        .map_err(Into::into);
        assert!(config.is_ok());
    }
}
