// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use codec::{
    v3::{
        ConnectAckPacket, ConnectPacket, PublishPacket, SubscribeAck, SubscribeAckPacket,
        SubscribePacket, UnsubscribePacket,
    },
    PacketId, QoS,
};
use tokio::sync::oneshot;

use crate::types::{ListenerId, SessionGid, SessionId, SessionInfo, Uptime};

use crate::session::CachedSession;

#[derive(Debug, Clone)]
pub enum ListenerToAuthCmd {
    /// session_gid, username, password
    RequestAuth(SessionGid, ConnectPacket),
}

#[derive(Debug, Clone)]
pub enum AuthToListenerCmd {
    /// session-id, access-granted
    ResponseAuth(SessionId, bool, ConnectPacket),
}

#[derive(Debug, Clone)]
pub enum AclToListenerCmd {
    /// (session_id, packet, accepted).
    PublishAck(SessionId, PublishPacket, bool),

    SubscribeAck(SessionId, SubscribePacket, Vec<SubscribeAck>, bool),
}

#[derive(Debug, Clone)]
pub enum ListenerToAclCmd {
    /// Check publish packet.
    Publish(SessionGid, PublishPacket),

    /// Check subscribe packet.
    Subscribe(SessionGid, SubscribePacket),
}

#[derive(Debug, Clone)]
pub enum ListenerToSessionCmd {
    /// Accepted or not.
    ConnectAck(ConnectAckPacket, Option<CachedSession>),

    /// Response to Publish packet.
    ///
    /// (packet_id, qos, accept) pair.
    PublishAck(PacketId, QoS, bool),

    Publish(PublishPacket),

    SubscribeAck(SubscribeAckPacket),

    /// Disconnect client connection.
    Disconnect,
}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Connect(SessionId, ConnectPacket),
    Publish(SessionId, PublishPacket),
    Subscribe(SessionId, SubscribePacket),
    Unsubscribe(SessionId, UnsubscribePacket),
    Disconnect(SessionId),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    CheckCachedSessionResp(SessionId, Option<CachedSession>),

    Publish(SessionId, PublishPacket),

    SubscribeAck(SessionId, SubscribeAckPacket),
}

#[derive(Debug, Clone)]
pub enum ListenerToDispatcherCmd {
    // client-id
    CheckCachedSession(SessionGid, String),

    Publish(PublishPacket),

    Subscribe(SessionGid, SubscribePacket),

    Unsubscribe(SessionGid, UnsubscribePacket),

    SessionAdded(ListenerId),
    SessionRemoved(ListenerId),
}

#[derive(Debug, Clone)]
pub enum DispatcherToMetricsCmd {
    /// listener id, listener address
    ListenerAdded(ListenerId, String),
    /// listener id
    ListenerRemoved(ListenerId),

    /// listener id, count
    SessionAdded(ListenerId, usize),
    /// listener id, count
    SessionRemoved(ListenerId, usize),

    /// listener id, count
    SubscriptionsAdded(ListenerId, usize),
    /// listener id, count
    SubscriptionsRemoved(ListenerId, usize),

    /// listener id, count, bytes
    RetainedMessageAdded(ListenerId, usize, usize),
    /// listener id, count, bytes
    RetainedMessageRemoved(ListenerId, usize, usize),

    /// listener id, count, bytes
    PublishPacketSent(ListenerId, usize, usize),
    /// listener id, count, bytes
    PublishPacketReceived(ListenerId, usize, usize),
    /// count, bytes
    PublishPacketDropped(usize, usize),

    /// listener id, count, bytes
    PacketSent(ListenerId, usize, usize),
    /// listener id, count, bytes
    PacketReceived(ListenerId, usize, usize),
}

#[derive(Debug, Clone)]
pub enum MetricsToDispatcherCmd {
    Publish(PublishPacket),
}

#[derive(Debug, Clone)]
pub enum DispatcherToBackendsCmd {
    /// session info
    SessionAdded(SessionInfo),

    /// listener id, session id
    SessionRemoved(ListenerId, SessionId),
}

#[derive(Debug, Clone)]
pub enum BackendsToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToBridgeCmd {}

#[derive(Debug, Clone)]
pub enum BridgeToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToGatewayCmd {}

#[derive(Debug, Clone)]
pub enum GatewayToDispatcherCmd {}

#[derive(Debug, Clone)]
pub enum DispatcherToRuleEngineCmd {}

#[derive(Debug, Clone)]
pub enum RuleEngineToDispatcherCmd {}

// Server context

#[derive(Debug)]
pub enum ServerContextToAclCmd {}

#[derive(Debug)]
pub enum ServerContextToAuthCmd {}

#[derive(Debug)]
pub enum ServerContextToBackendsCmd {}

#[derive(Debug)]
pub enum ServerContextToBridgeCmd {}

#[derive(Debug)]
pub enum ServerContextToGatewayCmd {}

#[derive(Debug)]
pub enum ServerContextToMetricsCmd {
    MetricsGetUptime(oneshot::Sender<Uptime>),
}

#[derive(Debug)]
pub enum ServerContextToRuleEngineCmd {}

#[derive(Debug)]
pub enum DashboardToServerContexCmd {
    MetricsGetUptime(oneshot::Sender<Uptime>),
}
