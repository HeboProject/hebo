// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Handles commands from server context.

use super::AuthApp;
use crate::commands::ServerContextToAuthCmd;

impl AuthApp {
    pub(super) async fn handle_server_ctx_cmd(&mut self, cmd: ServerContextToAuthCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}
