use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};
use tower::management_model::AddRoleReq;

use crate::entity::prelude::Role;
use crate::entity::role::{self};

pub struct AddRoleParam {
    pub role_id: String,
    pub role_name: String,
    pub remark: Option<String>,
}

impl From<AddRoleReq> for AddRoleParam {
    fn from(value: AddRoleReq) -> Self {
        let AddRoleReq {
            role_id,
            role_name,
            remark,
        } = value;
        Self {
            role_id,
            role_name,
            remark,
        }
    }
}

impl IntoActiveModel<role::ActiveModel> for AddRoleParam {
    fn into_active_model(self) -> role::ActiveModel {
        role::ActiveModel {
            role_id: to_av!(self.role_id),
            role_name: to_av!(self.role_name),
            remark: to_av!(self.remark),
            status: to_av!(YesNo::Yes.val()),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}

pub struct RoleManageService;

impl RoleManageService {
    pub async fn get_roles(db: &DbConn) -> Result<Vec<role::Model>, DbErr> {
        Role::find().all(db).await
    }

    pub async fn insert_role(db: &DbConn, param: AddRoleParam) -> Result<(), DbErr> {
        param.into_active_model().insert(db).await?;
        Ok(())
    }
}
