use tower::management_model::{AddGroupReq, GroupListItem, ManagementResource};
use tower::server_lib::prelude::*;

use crate::service::GroupManageService;
use crate::StateEx;

#[get("/list")]
pub async fn get_groups(
    config: StateEx,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    GroupManageService::get_groups(&config.conn)
        .await
        .into_resp_and(|groups| {
            groups
                .into_iter()
                .map(|group| GroupListItem {
                    group_id: group.group_id,
                    group_name: group.group_name,
                    remark: group.remark,
                    status: group.status.try_into().unwrap_or_default(),
                })
                .collect::<Vec<_>>()
        })
}

#[post("")]
pub async fn add_group(
    config: StateEx,
    _principal: TowerPrincipal,
    req: web::Json<AddGroupReq>,
) -> Result<HttpResponse, Error> {
    GroupManageService::insert_group(&config.conn, req.0.into())
        .await
        .into_resp()
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::Group.scope())
            .service(get_groups)
            .service(add_group),
    );
}
