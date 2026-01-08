use tower::{
    common::Urlable,
    management_model::{GetUserVipLevelResp, ManagementResource, SaveUserVipLevelReq},
    server_lib::prelude::*,
};

use crate::{
    service::{UserVipLevelManageService, UserVipLevelService},
    StateEx,
};

#[post("")]
pub async fn save_user_vip_level(
    _principal: TowerPrincipal,
    state: StateEx,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let req: SaveUserVipLevelReq = bin_req(body).await?;
    UserVipLevelManageService::save_vip_level(&state.conn, req.into())
        .await
        .into_resp()
}

#[get("/{user_id}")]
pub async fn get_user_vip_level(
    _principal: TowerPrincipal,
    state: StateEx,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    UserVipLevelManageService::get_vip_level_by_user(&state.conn, &user_id)
        .await
        .into_resp_and(|vip| {
            vip.map(|v| GetUserVipLevelResp {
                level_code: v.level_code.try_into().unwrap_or_default(),
                status: v.status.try_into().unwrap_or_default(),
                expried: v.expried,
            })
            .unwrap_or_default()
        })
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::UserVipLevel.scope())
            .service(save_user_vip_level)
            .service(get_user_vip_level),
    );
}
