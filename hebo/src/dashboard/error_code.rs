// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    OK = 0,

    /// Bad rpc.
    ///
    /// Channel connected to server_ctx is closed abnormally.
    BadRpc = 101,

    /// Unknown error.
    UnknownError = 102,

    /// Username or password error.
    UsernamePasswordError = 103,

    /// Empty username or password.
    EmptyUsernamePassword = 104,

    /// User does not exist.
    UserNotFound = 105,

    /// Admin can not be deleted.
    DeleteAdminDenied = 106,

    /// Missing request parameter.
    MissingRequiredParam = 107,

    /// Request parameter type error.
    ParamTypeError = 108,

    /// Request parameter is not a json.
    ParamInvalidJson = 109,

    /// Plugin has been loaded.
    PluginAlreadyLoaded = 110,

    /// Plugin has been unloaded.
    PluginAlreadyUnloaded = 111,

    /// User is not online.
    UserOffline = 112,
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::OK
    }
}
