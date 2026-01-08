use sea_orm::*;
use tower::{
    common::{to_av, utc_now_av, utc_now_av_opt},
    management_model::{AddDictItemReq, UpdateDictItemReq},
};

use crate::entity::{dict_item, prelude::DictItem};

pub struct AddDictItemParam {
    pub dict_code: String,
    pub item_name: String,
    pub item_value: String,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<AddDictItemReq> for AddDictItemParam {
    fn from(value: AddDictItemReq) -> Self {
        let AddDictItemReq {
            dict_code,
            item_name,
            item_value,
            public,
            remark,
            status,
        } = value;
        Self {
            dict_code,
            item_name,
            item_value,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

impl IntoActiveModel<dict_item::ActiveModel> for AddDictItemParam {
    fn into_active_model(self) -> dict_item::ActiveModel {
        dict_item::ActiveModel {
            dict_code: to_av!(self.dict_code),
            item_name: to_av!(self.item_name),
            item_value: to_av!(self.item_value),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UpdateDictItemParam {
    pub id: i32,
    pub item_name: String,
    pub item_value: String,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<UpdateDictItemReq> for UpdateDictItemParam {
    fn from(value: UpdateDictItemReq) -> Self {
        let UpdateDictItemReq {
            id,
            item_name,
            item_value,
            public,
            remark,
            status,
        } = value;
        Self {
            id,
            item_name,
            item_value,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

impl IntoActiveModel<dict_item::ActiveModel> for UpdateDictItemParam {
    fn into_active_model(self) -> dict_item::ActiveModel {
        dict_item::ActiveModel {
            id: Unchanged(self.id),
            item_name: to_av!(self.item_name),
            item_value: to_av!(self.item_value),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            updated: utc_now_av_opt!(),
            ..Default::default()
        }
    }
}
pub struct DictItemManageService;
impl DictItemManageService {
    pub async fn insert(db: &DbConn, param: AddDictItemParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.id)
    }

    pub async fn update(db: &DbConn, param: UpdateDictItemParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().update(db).await?;
        Ok(ret.id)
    }

    pub async fn list(db: &DbConn, dict_code: &str) -> Result<Vec<dict_item::Model>, DbErr> {
        DictItem::find()
            .filter(dict_item::Column::DictCode.eq(dict_code))
            .all(db)
            .await
    }
}
