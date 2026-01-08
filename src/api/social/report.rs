use std::collections::HashMap;

use actix_web::{post, web, Error, HttpResponse};
use tower::common::{
    social::GetReportsByIdsReq, social::GetReportsByIdsResp, social::InitReportReq, TowerResource,
    Urlable,
};
use tower::server_lib::to_resp;

use crate::service::SocialReportService;
use crate::StateEx;

/// 报表初始化
#[post("/init")]
pub async fn init_report(
    config: StateEx,
    req: web::Json<InitReportReq>,
) -> Result<HttpResponse, Error> {
    let ret = SocialReportService::insert(&config.conn, &req.target_id, req.target_type).await;

    to_resp(ret)
}

#[post("/list")]
pub async fn get_reports_by_ids(
    config: StateEx,
    req: web::Json<GetReportsByIdsReq>,
) -> Result<HttpResponse, Error> {
    let ret = SocialReportService::get_report_by_ids(
        &config.conn,
        req.target_ids.clone(),
        req.target_type,
    )
    .await;

    let ret = ret.map(|models| {
        models
            .iter()
            .map(|model| {
                (
                    model.target_id.clone(),
                    GetReportsByIdsResp {
                        target_id: model.target_id.clone(),
                        target_type: model.target_type,
                        like_count: model.like_count,
                        follow_count: model.follow_count,
                        favorite_count: model.favorite_count,
                        comment_count: model.comment_count,
                        score_count: model.score_count,
                        score_total: model.score_total,
                    },
                )
            })
            .collect::<HashMap<String, GetReportsByIdsResp>>()
    });
    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Report.scope())
            .service(init_report)
            .service(get_reports_by_ids),
    );
}
