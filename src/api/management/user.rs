use tower::management_model::{
    AddUserReq, ManagementResource, ResetPasswordReq, SetStatusReq, UserListReq, UserListResp,
};
use tower::server_lib::prelude::*;

use crate::service::{GetPagedUsersParam, UserManageService, UserService};
use crate::StateEx;

#[post("/page")]
pub async fn get_paged_users(
    config: StateEx,
    req: web::Json<Page<UserListReq>>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    let ret =
        UserManageService::get_paged_users(&config.conn, req.0.map(GetPagedUsersParam::from)).await;

    ret.into_resp_and(|models| {
        models.map(|models| {
            models
                .iter()
                .map(|model| UserListResp {
                    user_id: model.user_id.clone(),
                    username: model.username.clone(),
                    nickname: model.nickname.clone(),
                    status: model.status,
                    created: model.created,
                    updated: model.updated,
                    avatar: model.avatar,
                })
                .collect::<Vec<_>>()
        })
    })
}

#[post("")]
pub async fn add_user(
    _principal: TowerPrincipal,
    req: web::Json<AddUserReq>,
    state: StateEx,
) -> Result<HttpResponse, Error> {
    UserService::add_user(&state.conn, req.0.into())
        .await
        .into_resp()
}

#[post("/reset_password")]
pub async fn reset_password(
    _principal: TowerPrincipal,
    req: web::Json<ResetPasswordReq>,
    state: StateEx,
) -> Result<HttpResponse, Error> {
    UserManageService::reset_password(&state.conn, &req.user_id)
        .await
        .into_resp()
}

#[post("/status")]
pub async fn set_status(
    _principal: TowerPrincipal,
    req: web::Json<SetStatusReq>,
    state: StateEx,
) -> Result<HttpResponse, Error> {
    UserManageService::set_status(&state.conn, &req.user_id, req.status)
        .await
        .into_resp()
}
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::User.scope())
            .service(get_paged_users)
            .service(set_status)
            .service(reset_password),
    );
}
