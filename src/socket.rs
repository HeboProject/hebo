// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use std::net::UdpSocket;
use std::os::unix::io::{AsRawFd, RawFd};
use tokio::net::TcpListener;

use crate::error::{Error, ErrorKind};

fn bind_interface(socket_fd: RawFd, interface: &str) -> Result<(), Error> {
    if !interface.is_empty() {
        nc::setsockopt(
            socket_fd,
            nc::SOL_SOCKET,
            nc::SO_BINDTODEVICE,
            interface.as_ptr() as usize,
            interface.len() as u32,
        )
        .map_err(|errno| {
            Error::from_string(
                ErrorKind::KernelError,
                format!(
                    "Failed to bind interface: {}, err: {}",
                    interface,
                    nc::strerror(errno)
                ),
            )
        })?;
    }
    Ok(())
}

fn enable_fast_open(socket_fd: RawFd) -> Result<(), Error> {
    // For Linux, value is the queue length of pending packets.
    //
    // TODO(Shaohua): Add a config option
    #[cfg(target_os = "linux")]
    let queue_len: i32 = 5;
    // For the others, just a boolean value for enable and disable.
    #[cfg(not(target_os = "linux"))]
    let queue_len: i32 = 1;
    let queue_len_ptr = &queue_len as *const i32 as usize;

    // TODO(Shaohua): Replace with nc::TCP_FASTOPEN in new version.
    const TCP_FASTOPEN: i32 = 23;

    nc::setsockopt(
        socket_fd,
        nc::IPPROTO_TCP,
        TCP_FASTOPEN,
        queue_len_ptr,
        std::mem::size_of_val(&queue_len) as u32,
    )
    .map_err(|errno| {
        Error::from_string(
            ErrorKind::KernelError,
            format!(
                "Failed to enable socket fast open, got err: {}",
                nc::strerror(errno)
            ),
        )
    })
}

pub async fn new_tcp_listener(address: &str, interface: &str) -> Result<TcpListener, Error> {
    let listener = TcpListener::bind(address).await?;
    let socket_fd: RawFd = listener.as_raw_fd();

    bind_interface(socket_fd, interface)?;

    enable_fast_open(socket_fd)?;

    Ok(listener)
}

pub fn new_udp_socket(address: &str, interface: &str) -> Result<UdpSocket, Error> {
    let socket = UdpSocket::bind(address)?;
    let socket_fd: RawFd = socket.as_raw_fd();

    bind_interface(socket_fd, interface)?;

    Ok(socket)
}
