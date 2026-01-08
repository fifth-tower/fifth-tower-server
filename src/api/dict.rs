use std::collections::HashMap;

use actix_web::{get, web, Error, HttpResponse};
use multimap::MultiMap;
use sea_orm::{DbConn, DbErr};
use serde::Deserialize;
use tower::common::dict::DictData;
use tower::common::dict::DictItemData;
use tower::common::TowerResource;
use tower::common::Urlable;
use tower::common::YesNo;
use tower::server_lib::success_resp;

use crate::service::DictItemService;
use crate::service::DictService;
use crate::StateEx;

#[derive(Deserialize)]
struct GetDictsQuery {
    dict_code: String,
}

#[get("/list")]
pub async fn get_all_dicts(
    state: StateEx,
    req: web::Query<GetDictsQuery>,
) -> Result<HttpResponse, Error> {
    let dict_code = &req.dict_code;
    let ret: DictData = if dict_code.is_empty() {
        let dicts = state.dicts.lock().unwrap();
        dicts.clone()
    } else {
        let dicts = state.dicts.lock().unwrap();
        dicts.get_dict(&req.dict_code)
    };
    success_resp(ret)
}

pub async fn get_dicts_data(db: &DbConn) -> Result<DictData, DbErr> {
    let items = DictItemService::get_all(db).await?;
    let item_map: MultiMap<String, DictItemData> = items
        .iter()
        .map(|dict| {
            (
                dict.dict_code.clone(),
                DictItemData {
                    name: dict.item_name.clone(),
                    value: dict.item_value.clone(),
                    public: dict.public.try_into().unwrap_or(YesNo::No),
                },
            )
        })
        .collect();

    let dicts = DictService::get_all(db).await?;
    let mut dict_map: HashMap<String, DictData> = dicts
        .iter()
        .map(|dict| {
            (
                dict.dict_code.clone(),
                DictData {
                    code: dict.dict_code.clone(),
                    name: dict.dict_name.clone(),
                    public: dict.public.try_into().unwrap_or(YesNo::No),
                    data: item_map.get_vec(&dict.dict_code).cloned().unwrap_or(vec![]),
                    children: vec![],
                },
            )
        })
        .collect();
    for dict in dicts.iter() {
        if let Some(parent_code) = &dict.parent_code {
            if let (Some(child), Some(parent)) = (
                dict_map.get(&dict.dict_code).cloned(),
                dict_map.get_mut(parent_code),
            ) {
                parent.add_child(child);
                dict_map.remove(&dict.dict_code);
            }
        }
    }

    let mut config = DictData::new();
    config.children = dict_map.values().cloned().collect();

    Ok(config)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(TowerResource::Dict.scope()).service(get_all_dicts));
}
