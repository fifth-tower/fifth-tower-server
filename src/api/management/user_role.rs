use tower::management_model::{GetUserRolesReq, ManagementResource, SetUserRolesReq};
use tower::server_lib::prelude::*;

use crate::service::UserRoleManageService;
use crate::StateEx;

#[put("")]
pub async fn set_user_roles(
    config: StateEx,
    req: web::Json<SetUserRolesReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    UserRoleManageService::set_user_roles(&config.conn, req.0.into())
        .await
        .into_resp()
}

#[get("/list")]
pub async fn get_user_roles(
    config: StateEx,
    req: web::Query<GetUserRolesReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    let roles = UserRoleManageService::get_user_roles(&config.conn, &req.user_id)
        .await
        .into_error()?;
    success_resp(roles)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::UserRole.scope())
            .service(set_user_roles)
            .service(get_user_roles),
    );
}
