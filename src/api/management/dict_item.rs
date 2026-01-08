use actix_web::HttpRequest;
use tower::management_model::{
    AddDictItemReq, DictItemListItem, DictItemListReq, ManagementResource, UpdateDictItemReq,
};
use tower::server_lib::prelude::*;

use crate::api::management::sync_util::sync_dict;
use crate::entity::dict_item;
use crate::service::DictItemManageService;
use crate::StateEx;

#[post("")]
pub async fn add_dict_item(
    state: StateEx,
    req: web::Json<AddDictItemReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    DictItemManageService::insert(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_dict(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[post("/one")]
pub async fn update_dict_item(
    state: StateEx,
    req: web::Json<UpdateDictItemReq>,
    _principal: TowerPrincipal,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    DictItemManageService::update(&state.conn, req.0.into())
        .await
        .into_error()?;
    sync_dict(&state, get_bearer_auth_from_header(&request))
        .await
        .into_resp()
}

#[get("/list")]
pub async fn get_dict_items(
    state: StateEx,
    req: web::Query<DictItemListReq>,
    _principal: TowerPrincipal,
) -> Result<HttpResponse, Error> {
    DictItemManageService::list(&state.conn, &req.0.dict_code)
        .await
        .into_resp_and(|dict_items| {
            dict_items
                .into_iter()
                .map(
                    |dict_item::Model {
                         id,
                         dict_code,
                         item_name,
                         item_value,
                         public,
                         remark,
                         status,
                         created,
                         updated,
                     }| DictItemListItem {
                        id,
                        dict_code,
                        item_name,
                        item_value,
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

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ManagementResource::DictItem.scope())
            .service(get_dict_items)
            .service(add_dict_item)
            .service(update_dict_item),
    );
}
