use actix_web::HttpRequest;
use tower::management_model::{AddDictReq, DictListItem, ManagementResource, UpdateDictReq};
use tower::server_lib::prelude::*;

use crate::api::management::sync_util::sync_dict;
use crate::service::DictManageService;
use crate::StateEx;

#[post("")]
pub async fn add_dict(
    state: StateEx,
    req: web::Json<AddDictReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    DictManageService::insert(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_dict(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[post("/one")]
pub async fn update_dict(
    state: StateEx,
    req: web::Json<UpdateDictReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    DictManageService::update(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_dict(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[get("/list")]
pub async fn get_dicts(
    state: StateEx,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    DictManageService::list(&state.conn)
        .await
        .into_resp_and(|dicts| {
            dicts
                .into_iter()
                .map(|dict| DictListItem {
                    dict_code: dict.dict_code,
                    dict_name: dict.dict_name,
                    parent_code: dict.parent_code,
                    remark: dict.remark,
                    status: YesNo::try_from(dict.status).unwrap(),
                    public: YesNo::try_from(dict.public).unwrap(),
                    children: vec![],
                    created: dict.created,
                    updated: dict.updated,
                })
                .collect::<Vec<_>>()
        })
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::Dict.scope())
            .service(get_dicts)
            .service(add_dict)
            .service(update_dict),
    );
}
