use actix_web::{post, web, Error, HttpResponse};
use tower::{
    common::{user::AddFeedBackReq, TowerResource, Urlable},
    server_lib::{TowerPrincipal, TransferExt},
};

use crate::{
    service::{AddFeedBackParam, UserFeedBackService},
    StateEx,
};

#[post("")]
pub async fn add_feed_back(
    state: StateEx,
    principal: TowerPrincipal,
    body: web::Json<AddFeedBackReq>,
) -> Result<HttpResponse, Error> {
    UserFeedBackService::add_feed_back(
        &state.conn,
        AddFeedBackParam::from(body.0.into(), &principal.user_id),
    )
    .await
    .into_resp()
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(TowerResource::UserFeedBack.scope()).service(add_feed_back));
}
