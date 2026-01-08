use actix_web::{get, post, web, Error, HttpResponse};
use tower::common::{social::MyScoresReq, social::ScoreReq, TowerResource, Urlable};
use tower::server_lib::{to_resp, TowerPrincipal};

use crate::service::{ScoreParam, ScoreService};
use crate::StateEx;

/// 评分
#[post("")]
pub async fn score(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<ScoreReq>,
) -> Result<HttpResponse, Error> {
    let ret = ScoreService::score(&config.conn, ScoreParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}

#[get("/list/my")]
pub async fn get_my_scores(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Query<MyScoresReq>,
) -> Result<HttpResponse, Error> {
    let ret = ScoreService::get_my_scores(&config.conn, &principal.user_id, req.target_type).await;

    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(TowerResource::Score.scope()).service(score));
}
