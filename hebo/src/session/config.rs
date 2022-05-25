// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    keep_alive: Duration,
    connect_timeout: Duration,

    maximum_inflight_messages: usize,
    maximum_packet_size: usize,

    allow_empty_client_id: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            keep_alive: Duration::from_secs(10),
            connect_timeout: Duration::from_secs(10),

            maximum_inflight_messages: 10,
            maximum_packet_size: 10,

            allow_empty_client_id: false,
        }
    }

    pub fn set_keep_alive(&mut self, keep_alive: u32) -> &mut Self {
        self.keep_alive = Duration::from_secs(u64::from(keep_alive));
        self
    }

    #[inline]
    #[must_use]
    pub const fn keep_alive(&self) -> Duration {
        self.keep_alive
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: u32) -> &mut Self {
        self.connect_timeout = Duration::from_secs(u64::from(connect_timeout));
        self
    }

    #[inline]
    #[must_use]
    pub const fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    pub fn set_maximum_inflight_messages(&mut self, maximum_inflight_messages: u32) -> &mut Self {
        self.maximum_inflight_messages = maximum_inflight_messages as usize;
        self
    }

    #[inline]
    #[must_use]
    pub const fn maximum_inflight_messages(&self) -> usize {
        self.maximum_inflight_messages
    }

    pub fn set_maximum_packet_size(&mut self, maximum_packet_size: u32) -> &mut Self {
        self.maximum_packet_size = maximum_packet_size as usize;
        self
    }

    #[inline]
    #[must_use]
    pub const fn maximum_packet_size(&self) -> usize {
        self.maximum_packet_size
    }

    pub fn set_allow_empty_client_id(&mut self, allow_empty_client_id: bool) -> &mut Self {
        self.allow_empty_client_id = allow_empty_client_id;
        self
    }

    #[inline]
    #[must_use]
    pub const fn allow_empty_client_id(&self) -> bool {
        self.allow_empty_client_id
    }
}
