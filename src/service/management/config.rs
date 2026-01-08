use sea_orm::*;
use tower::{
    common::{to_av, utc_now_av, utc_now_av_opt, YesNo},
    management_model::{AddConfigReq, ConfigListReq, UpdateConfigReq},
};

use crate::entity::{config, prelude::Config};

pub struct AddConfigParam {
    pub app_id: String,
    pub app_version: Option<String>,
    pub var_name: String,
    pub var_value: String,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<AddConfigReq> for AddConfigParam {
    fn from(value: AddConfigReq) -> Self {
        let AddConfigReq {
            app_id,
            app_version,
            var_name,
            var_value,
            public,
            remark,
            status,
        } = value;
        Self {
            app_id,
            app_version,
            var_name,
            var_value,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

impl IntoActiveModel<config::ActiveModel> for AddConfigParam {
    fn into_active_model(self) -> config::ActiveModel {
        config::ActiveModel {
            app_id: to_av!(self.app_id),
            app_version: to_av!(self.app_version),
            var_name: to_av!(self.var_name),
            var_value: to_av!(self.var_value),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UpdateConfigParam {
    pub id: i32,
    pub app_id: String,
    pub app_version: Option<String>,
    pub var_name: String,
    pub var_value: String,
    pub public: i16,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<UpdateConfigReq> for UpdateConfigParam {
    fn from(value: UpdateConfigReq) -> Self {
        let UpdateConfigReq {
            id,
            app_id,
            app_version,
            var_name,
            var_value,
            public,
            remark,
            status,
        } = value;
        Self {
            id,
            app_id,
            app_version,
            var_name,
            var_value,
            public: public.val(),
            remark,
            status: status.val(),
        }
    }
}

pub struct ConfigListParam {
    pub app_id: String,
    pub var_name: Option<String>,
    pub status: Option<YesNo>,
}
impl From<ConfigListReq> for ConfigListParam {
    fn from(value: ConfigListReq) -> Self {
        Self {
            app_id: value.app_id,
            var_name: if value.var_name.len() > 0 {
                Some(value.var_name)
            } else {
                None
            },
            status: value.status,
        }
    }
}

impl IntoActiveModel<config::ActiveModel> for UpdateConfigParam {
    fn into_active_model(self) -> config::ActiveModel {
        config::ActiveModel {
            id: Unchanged(self.id),
            app_id: to_av!(self.app_id),
            app_version: to_av!(self.app_version),
            var_name: to_av!(self.var_name),
            var_value: to_av!(self.var_value),
            public: to_av!(self.public),
            remark: to_av!(self.remark),
            status: to_av!(self.status),
            updated: utc_now_av_opt!(),
            ..Default::default()
        }
    }
}
pub struct ConfigManageService;
impl ConfigManageService {
    pub async fn insert(db: &DbConn, param: AddConfigParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.id)
    }

    pub async fn update(db: &DbConn, param: UpdateConfigParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().update(db).await?;
        Ok(ret.id)
    }

    pub async fn list(db: &DbConn, param: ConfigListParam) -> Result<Vec<config::Model>, DbErr> {
        Config::find()
            .filter(config::Column::AppId.eq(param.app_id))
            .apply_if(param.var_name, |query, val| {
                query.filter(config::Column::VarName.starts_with(val))
            })
            .apply_if(param.status, |query, val| {
                query.filter(config::Column::Status.eq(val.val()))
            })
            .all(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let _ = Config::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
