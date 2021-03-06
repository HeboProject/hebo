// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use warp::Filter;

use super::metrics;
use super::types::DashboardSender;

pub fn init(
    sender: DashboardSender,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let sender_filter = warp::any().map(move || sender.clone());

    warp::get()
        .and(warp::path("api"))
        .and(warp::path("v1"))
        .and(warp::path("metrics"))
        .and(warp::path("uptime"))
        .and(warp::path::end())
        .and(sender_filter)
        .and_then(metrics::get_uptime)
}
