// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

use crate::session::CachedSession;

#[derive(Debug)]
pub struct CachedSessions {
    map: HashMap<String, CachedSession>,
}

impl CachedSessions {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
