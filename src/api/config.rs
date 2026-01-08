use actix_web::{get, web, Error, HttpResponse};
use sea_orm::{DbConn, DbErr};
use tower::common::App;
use tower::common::TowerResource;
use tower::common::Urlable;
use tower::common::YesNo;
use tower::config_model::GetConfigByKeyQuery;
use tower::config_model::GetConfigsQuery;
use tower::config_model::{Config, ConfigData};
use tower::server_lib::to_resp;
use tower::server_lib::TransferExt;

use crate::service::ConfigService;
use crate::StateEx;

#[get("/internal")]
pub async fn get_configs(
    config: StateEx,
    req: web::Query<GetConfigsQuery>,
) -> Result<HttpResponse, Error> {
    let ret = get_configs_by_app(&config.conn, &req.app_name, None).await;
    to_resp(ret)
}

#[get("/public")]
pub async fn get_public_configs(
    config: StateEx,
    req: web::Query<GetConfigsQuery>,
) -> Result<HttpResponse, Error> {
    let ret = get_configs_by_app(&config.conn, &req.app_name, Some(YesNo::Yes)).await;
    to_resp(ret)
}

#[get("/key")]
pub async fn get_public_config_by_key(
    config: StateEx,
    req: web::Query<GetConfigByKeyQuery>,
) -> Result<HttpResponse, Error> {
    ConfigService::get_by_key(&config.conn, &req.app_name, &req.config_key, YesNo::Yes)
        .await
        .into_resp_and(|config| config.map(|f| f.var_value))
}

async fn get_configs_by_app(
    db: &DbConn,
    app_name: &str,
    public: Option<YesNo>,
) -> Result<Config, DbErr> {
    let ret = ConfigService::list(db, app_name, public).await;
    ret.map(|configs| {
        let mut config = Config::new();
        let configs = configs
            .iter()
            .map(|c| ConfigData {
                name: c.var_name.to_owned(),
                value: c.var_value.to_owned(),
                version: c.app_version.as_ref().map(|v| v.to_owned()),
                public: c.public.try_into().unwrap(),
            })
            .collect::<Vec<ConfigData>>();
        config.set_data(configs);
        config
    })
}

pub async fn load_configs(db: &DbConn, app: &App) -> Result<Config, DbErr> {
    let app_name = app.to_string();
    get_configs_by_app(db, &app_name, None).await
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Config.scope())
            .service(get_configs)
            .service(get_public_configs)
            .service(get_public_config_by_key),
    );
}
