use sea_orm::*;
use tower::{
    common::{to_av, utc_now_av, utc_now_av_opt},
    management_model::{AddDictReq, UpdateDictReq},
};

use crate::entity::dict;
use crate::entity::prelude::Dict;

pub struct AddDictParam {
    pub dict_code: String,
    pub dict_name: String,
    pub parent_code: Option<String>,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<AddDictReq> for AddDictParam {
    fn from(value: AddDictReq) -> Self {
        let AddDictReq {
            dict_code,
            dict_name,
            parent_code,
            public,
            remark,
            status,
        } = value;
        Self {
            dict_code,
            dict_name,
            parent_code,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

impl IntoActiveModel<dict::ActiveModel> for AddDictParam {
    fn into_active_model(self) -> dict::ActiveModel {
        dict::ActiveModel {
            dict_code: to_av!(self.dict_code),
            dict_name: to_av!(self.dict_name),
            parent_code: to_av!(self.parent_code),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UpdateDictParam {
    pub dict_code: String,
    pub dict_name: String,
    pub parent_code: Option<String>,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<UpdateDictReq> for UpdateDictParam {
    fn from(value: UpdateDictReq) -> Self {
        let UpdateDictReq {
            dict_code,
            dict_name,
            parent_code,
            public,
            remark,
            status,
        } = value;
        Self {
            dict_code,
            dict_name,
            parent_code,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

impl IntoActiveModel<dict::ActiveModel> for UpdateDictParam {
    fn into_active_model(self) -> dict::ActiveModel {
        dict::ActiveModel {
            dict_code: Unchanged(self.dict_code),
            dict_name: to_av!(self.dict_name),
            parent_code: to_av!(self.parent_code),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            updated: utc_now_av_opt!(),
            ..Default::default()
        }
    }
}
pub struct DictManageService;
impl DictManageService {
    pub async fn insert(db: &DbConn, param: AddDictParam) -> Result<String, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.dict_code)
    }

    pub async fn update(db: &DbConn, param: UpdateDictParam) -> Result<String, DbErr> {
        let ret = param.into_active_model().update(db).await?;
        Ok(ret.dict_code)
    }
    pub async fn list(db: &DbConn) -> Result<Vec<dict::Model>, DbErr> {
        Dict::find().all(db).await
    }
}
