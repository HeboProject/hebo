// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

//! Backends app handlers

use codec::PublishPacket;

use super::Dispatcher;
use crate::commands::BackendsToDispatcherCmd;

impl Dispatcher {
    /// Send packet to backends.
    pub(super) async fn backends_store_packet(&mut self, packet: &PublishPacket) {
        log::info!("backends store packet: {:?}", packet);
    }

    pub(super) async fn handle_backends_cmd(&mut self, cmd: BackendsToDispatcherCmd) {
        log::info!("cmd: {:?}", cmd);
    }
}