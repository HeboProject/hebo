// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{PublishPacket, SubscribePacket, UnsubscribePacket};

pub type ConnectionId = u64;

#[derive(Clone, Debug)]
pub enum ListenerCommand {
    Publish(PublishPacket),
}

#[derive(Debug)]
pub enum SessionCommand {
    Publish(PublishPacket),
    Subscribe(ConnectionId, SubscribePacket),
    Unsubscribe(ConnectionId, UnsubscribePacket),
    Disconnect(ConnectionId),
}

#[derive(Debug)]
pub enum StorageCommand {
    Publish(PublishPacket),
}
