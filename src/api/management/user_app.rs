use tower::management_model::{GetUserAppsReq, ManagementResource, SetUserAppsReq};
use tower::server_lib::prelude::*;

use crate::service::UserAppManageService;
use crate::StateEx;

#[put("")]
pub async fn set_user_apps(
    config: StateEx,
    req: web::Json<SetUserAppsReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    UserAppManageService::set_user_apps(&config.conn, req.0.into())
        .await
        .into_resp()
}

#[get("/list")]
pub async fn get_user_apps(
    config: StateEx,
    req: web::Query<GetUserAppsReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    let apps = UserAppManageService::get_user_apps(&config.conn, &req.user_id)
        .await
        .into_error()?;
    success_resp(apps)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::UserApp.scope())
            .service(set_user_apps)
            .service(get_user_apps),
    );
}
