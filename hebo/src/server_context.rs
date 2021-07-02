// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::commands::{ConnectionCommand, ConnectionId, ServerCommand};
use crate::config::Config;
use crate::connection_context::ConnectionContext;
use crate::error::Error;
use crate::listeners;
use crate::sys_messages::SysMessage;
use codec::{PublishPacket, QoS, SubscribePacket, Topic, UnsubscribePacket};

#[derive(Debug)]
pub struct ServerContext {
    config: Config,

    pipelines: Vec<Pipeline>,
    connection_rx: Receiver<ConnectionCommand>,
    connection_tx: Sender<ConnectionCommand>,
    current_connection_id: ConnectionId,
    sys_message: SysMessage,
}

impl ServerContext {
    pub fn new(config: Config) -> ServerContext {
        let (connection_tx, connection_rx) = mpsc::channel(10);
        ServerContext {
            config,
            pipelines: Vec::new(),
            connection_rx,
            connection_tx,
            current_connection_id: 0,
            sys_message: SysMessage::new(),
        }
    }

    pub async fn run_loop(&mut self) -> Result<(), Error> {
        let listener = listeners::Listener::bind(&self.config.listeners[0]).await?;
        loop {
            tokio::select! {
                Ok(stream) = listener.accept() => {
                    self.new_connection(stream).await;
                },
                Some(cmd) = self.connection_rx.recv() => {
                    self.route_cmd(cmd).await;
                },
            }
        }
    }

    async fn new_connection(&mut self, stream: listeners::Stream) {
        let (server_tx, server_rx) = mpsc::channel(10);
        let connection_id = self.next_connection_id();
        let pipeline = Pipeline::new(server_tx, connection_id);
        self.pipelines.push(pipeline);
        let connection =
            ConnectionContext::new(stream, connection_id, self.connection_tx.clone(), server_rx);
        tokio::spawn(connection.run_loop());
    }

    async fn route_cmd(&mut self, cmd: ConnectionCommand) {
        match cmd {
            ConnectionCommand::Publish(packet) => self.on_publish(packet).await,
            ConnectionCommand::Subscribe(connection_id, packet) => {
                self.on_subscribe(connection_id, packet);
            }
            ConnectionCommand::Unsubscribe(connection_id, packet) => {
                self.on_unsubscribe(connection_id, packet)
            }
            ConnectionCommand::Disconnect(connection_id) => {
                if let Some(pos) = self
                    .pipelines
                    .iter()
                    .position(|pipe| pipe.connection_id == connection_id)
                {
                    log::debug!("Remove pipeline: {}", connection_id);
                    self.pipelines.remove(pos);
                }
            }
        }
    }

    fn next_connection_id(&mut self) -> ConnectionId {
        self.current_connection_id += 1;
        self.current_connection_id
    }

    fn on_subscribe(&mut self, connection_id: ConnectionId, packet: SubscribePacket) {
        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                for topic in packet.topics() {
                    // TODO(Shaohua): Returns error
                    match Topic::parse(topic.topic()) {
                        Ok(pattern) => pipeline.topics.push(SubscribedTopic {
                            pattern,
                            qos: topic.qos(),
                        }),
                        Err(err) => log::error!("Invalid sub topic: {:?}, err: {:?}", topic, err),
                    }
                }
                break;
            }
        }
    }

    fn on_unsubscribe(&mut self, connection_id: ConnectionId, packet: UnsubscribePacket) {
        for pipeline in self.pipelines.iter_mut() {
            if pipeline.connection_id == connection_id {
                pipeline
                    .topics
                    .retain(|ref topic| !packet.topics().any(|t| t == topic.pattern.topic()));
            }
            break;
        }
    }

    async fn on_publish(&mut self, packet: PublishPacket) {
        let cmd = ServerCommand::Publish(packet.clone());
        // TODO(Shaohua): Replace with a trie tree and a hash table.
        for pipeline in self.pipelines.iter_mut() {
            if topic_match(&pipeline.topics, packet.topic()) {
                if let Err(err) = pipeline.server_tx.send(cmd.clone()).await {
                    log::warn!("Failed to send publish packet to connection: {}", err);
                }
            }
        }
    }
}

fn topic_match(topics: &[SubscribedTopic], topic_str: &str) -> bool {
    for topic in topics {
        if topic.pattern.is_match(topic_str) {
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct SubscribedTopic {
    pattern: Topic,
    qos: QoS,
}

#[derive(Debug)]
pub struct Pipeline {
    server_tx: Sender<ServerCommand>,
    topics: Vec<SubscribedTopic>,
    connection_id: ConnectionId,
}

impl Pipeline {
    pub fn new(server_tx: Sender<ServerCommand>, connection_id: ConnectionId) -> Pipeline {
        Pipeline {
            server_tx,
            topics: Vec::new(),
            connection_id,
        }
    }
}
