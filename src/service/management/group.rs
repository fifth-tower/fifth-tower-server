use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};
use tower::management_model::AddGroupReq;

use crate::entity::group::{self};
use crate::entity::prelude::Group;
pub struct AddGroupParam {
    pub group_id: String,
    pub group_name: String,
    pub remark: Option<String>,
}

impl From<AddGroupReq> for AddGroupParam {
    fn from(value: AddGroupReq) -> Self {
        let AddGroupReq {
            group_id,
            group_name,
            remark,
        } = value;
        Self {
            group_id,
            group_name,
            remark,
        }
    }
}

impl IntoActiveModel<group::ActiveModel> for AddGroupParam {
    fn into_active_model(self) -> group::ActiveModel {
        group::ActiveModel {
            group_id: to_av!(self.group_id),
            group_name: to_av!(self.group_name),
            remark: to_av!(self.remark),
            status: to_av!(YesNo::Yes.val()),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}

pub struct GroupManageService;

impl GroupManageService {
    pub async fn get_groups(db: &DbConn) -> Result<Vec<group::Model>, DbErr> {
        Group::find().all(db).await
    }

    pub async fn insert_group(db: &DbConn, param: AddGroupParam) -> Result<(), DbErr> {
        param.into_active_model().insert(db).await?;
        Ok(())
    }
}
