use actix_web::{delete, post, web, Error, HttpResponse};
use tower::common::{social::LikeReq, TowerResource, Urlable};
use tower::server_lib::{to_resp, TowerPrincipal};

use crate::service::{LikeParam, LikeService};
use crate::StateEx;

/// 点赞
#[post("")]
pub async fn like(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<LikeReq>,
) -> Result<HttpResponse, Error> {
    let ret = LikeService::like(&config.conn, LikeParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}
/// 取消点赞
#[delete("/many")]
pub async fn unlike(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<LikeReq>,
) -> Result<HttpResponse, Error> {
    let ret = LikeService::unlike(&config.conn, LikeParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Like.scope())
            .service(like)
            .service(unlike),
    );
}
