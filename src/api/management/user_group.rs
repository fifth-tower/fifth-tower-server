use tower::management_model::{GetUserGroupsReq, ManagementResource, SetUserGroupsReq};
use tower::server_lib::prelude::*;

use crate::service::UserGroupManageService;
use crate::StateEx;

#[put("")]
pub async fn set_user_groups(
    config: StateEx,
    req: web::Json<SetUserGroupsReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    UserGroupManageService::set_user_groups(&config.conn, req.0.into())
        .await
        .into_resp()
}

#[get("/list")]
pub async fn get_user_groups(
    config: StateEx,
    req: web::Query<GetUserGroupsReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    let user_groups = UserGroupManageService::get_user_groups(&config.conn, &req.user_id)
        .await
        .into_error()?;
    success_resp(user_groups)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::UserGroup.scope())
            .service(set_user_groups)
            .service(get_user_groups),
    );
}
