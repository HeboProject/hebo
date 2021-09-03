// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use super::file_auth::FileAuth;
use crate::commands::{AuthToListenerCmd, ListenerId, ListenerToAuthCmd, SessionId};
use crate::config::Security;
use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct AuthApp {
    allow_anonymous: bool,
    file_auth: Option<FileAuth>,

    listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
    listener_receiver: Receiver<ListenerToAuthCmd>,
}

impl AuthApp {
    pub fn new(
        security: Security,
        listener_senders: Vec<(ListenerId, Sender<AuthToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToAuthCmd>,
    ) -> Result<Self, Error> {
        let file_auth = if let Some(password_file) = security.password_file {
            Some(FileAuth::new(password_file)?)
        } else {
            None
        };

        Ok(Self {
            allow_anonymous: security.allow_anonymous,
            file_auth,

            listener_senders,
            listener_receiver,
        })
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle listener cmd: {:?}", err);
                    }
                },
            }
        }
    }

    async fn handle_listener_cmd(&mut self, cmd: ListenerToAuthCmd) -> Result<(), Error> {
        log::info!("AuthApp::handle_listener_cmd(), cmd: {:?}", cmd);
        match cmd {
            ListenerToAuthCmd::RequestAuth(listener_id, session_id, username, password) => {
                self.on_listener_request_auth(listener_id, session_id, username, password)
                    .await
            }
        }
    }

    async fn on_listener_request_auth(
        &mut self,
        listener_id: ListenerId,
        session_id: SessionId,
        username: String,
        password: Vec<u8>,
    ) -> Result<(), Error> {
        let access_granted = if username.is_empty() {
            self.allow_anonymous
        } else if let Some(file_auth) = &self.file_auth {
            file_auth.is_match(&username, &password)?
        } else {
            false
        };
        for (sender_listener_id, sender) in &self.listener_senders {
            if *sender_listener_id == listener_id {
                let cmd = AuthToListenerCmd::ResponseAuth(session_id, access_granted);
                sender.send(cmd).await?;
                return Ok(());
            }
        }

        Err(Error::from_string(
            ErrorKind::ChannelError,
            format!(
                "AuthApp: Failed to find listener_senders with id: {}",
                listener_id
            ),
        ))
    }
}