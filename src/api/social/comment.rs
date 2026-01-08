use actix_web::{post, web, Error, HttpResponse};
use tower::common::{social::CommentReq, Urlable};
use tower::server_lib::TowerPrincipal;
use tower::{common::TowerResource, server_lib::to_resp};

use crate::service::{CommentParam, CommentService};
use crate::StateEx;

/// 评论
#[post("")]
pub async fn comment(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<CommentReq>,
) -> Result<HttpResponse, Error> {
    let ret =
        CommentService::comment(&config.conn, CommentParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(TowerResource::Comment.scope()).service(comment));
}
