use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};

use crate::entity::prelude::UserGroup;
use crate::entity::user_group;

#[derive(Debug)]
pub struct AddGroupParam {
    pub user_id: String,
    pub group_id: String,
}

impl IntoActiveModel<user_group::ActiveModel> for AddGroupParam {
    fn into_active_model(self) -> user_group::ActiveModel {
        user_group::ActiveModel {
            user_id: to_av!(self.user_id),
            group_id: to_av!(self.group_id),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UserGroupService;

impl UserGroupService {
    pub async fn add_group(db: &DbConn, param: AddGroupParam) -> Result<String, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.group_id)
    }

    pub async fn get_group(db: &DbConn, user_id: &str) -> Result<Option<user_group::Model>, DbErr> {
        UserGroup::find_by_id(user_id)
            .filter(user_group::Column::Status.eq(YesNo::Yes.val()))
            .one(db)
            .await
    }
}
