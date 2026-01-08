use tower::management_model::{AddRoleReq, ManagementResource, RoleListItem};
use tower::server_lib::prelude::*;

use crate::service::RoleManageService;
use crate::StateEx;

#[get("/list")]
pub async fn get_roles(config: StateEx, _principal: TowerPrincipal) -> Result<HttpResponse, Error> {
    RoleManageService::get_roles(&config.conn)
        .await
        .into_resp_and(|roles| {
            roles
                .into_iter()
                .map(|role| RoleListItem {
                    role_id: role.role_id,
                    role_name: role.role_name,
                    remark: role.remark,
                    status: role.status.try_into().unwrap_or_default(),
                })
                .collect::<Vec<_>>()
        })
}

#[post("")]
pub async fn add_role(
    config: StateEx,
    _principal: TowerPrincipal,
    req: web::Json<AddRoleReq>,
) -> Result<HttpResponse, Error> {
    RoleManageService::insert_role(&config.conn, req.0.into())
        .await
        .into_resp()
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::Role.scope())
            .service(get_roles)
            .service(add_role),
    );
}
