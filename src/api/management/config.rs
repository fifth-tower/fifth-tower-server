use std::str::FromStr;

use actix_web::HttpRequest;
use tower::management_model::{
    AddConfigReq, ConfigListItem, ConfigListReq, ManagementResource, UpdateConfigReq,
};
use tower::server_lib::prelude::*;

use crate::api::management::sync_util::sync_config;
use crate::entity::config;
use crate::service::ConfigManageService;
use crate::StateEx;

#[post("")]
pub async fn add_config(
    state: StateEx,
    req: web::Json<AddConfigReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    ConfigManageService::insert(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_config(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[post("/one")]
pub async fn update_config(
    state: StateEx,
    req: web::Json<UpdateConfigReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    ConfigManageService::update(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_config(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[post("/list")]
pub async fn get_configs(
    config: StateEx,
    req: web::Json<ConfigListReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    ConfigManageService::list(&config.conn, req.0.into())
        .await
        .into_resp_and(|configs| {
            configs
                .into_iter()
                .map(
                    |config::Model {
                         id,
                         app_id,
                         var_name,
                         var_value,
                         public,
                         remark,
                         status,
                         created,
                         updated,
                         app_version,
                     }| ConfigListItem {
                        id,
                        app_version,
                        app: App::from_str(&app_id).unwrap(),
                        var_name,
                        var_value,
                        public: YesNo::try_from(public).unwrap(),
                        remark,
                        status: YesNo::try_from(status).unwrap(),
                        created,
                        updated,
                    },
                )
                .collect::<Vec<_>>()
        })
}
#[delete("/{id}")]
pub async fn delete_config(
    config: StateEx,
    _principal: TowerPrincipal,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    ConfigManageService::delete(&config.conn, id.into_inner())
        .await
        .into_resp()
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::Config.scope())
            .service(get_configs)
            .service(add_config)
            .service(delete_config)
            .service(update_config),
    );
}
