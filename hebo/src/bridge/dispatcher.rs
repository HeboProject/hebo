// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use super::BridgeApp;
use crate::commands::DispatcherToBridgeCmd;
use crate::error::Error;

impl BridgeApp {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToBridgeCmd,
    ) -> Result<(), Error> {
        log::info!("cmd: {:?}", cmd);
        Ok(())
    }
}
