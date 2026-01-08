use actix_web::{delete, post, web, Error, HttpResponse};
use tower::common::{social::FollowReq, Urlable};
use tower::server_lib::TowerPrincipal;
use tower::{common::TowerResource, server_lib::to_resp};

use crate::service::{FollowParam, FollowService};
use crate::StateEx;

/// 关注
#[post("")]
pub async fn follow(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<FollowReq>,
) -> Result<HttpResponse, Error> {
    let ret = FollowService::follow(&config.conn, FollowParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}
/// 取消关注
#[delete("/many")]
pub async fn unfollow(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<FollowReq>,
) -> Result<HttpResponse, Error> {
    let ret =
        FollowService::unfollow(&config.conn, FollowParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Follow.scope())
            .service(follow)
            .service(unfollow),
    );
}
