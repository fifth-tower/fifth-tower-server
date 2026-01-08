use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};
use tower::management_model::SetUserGroupsReq;

use crate::entity::prelude::UserGroup;
use crate::entity::user_group;

#[derive(Debug)]
pub struct SetUserGroupsParam {
    pub user_id: String,
    pub group_ids: Vec<String>,
}

impl From<SetUserGroupsReq> for SetUserGroupsParam {
    fn from(value: SetUserGroupsReq) -> Self {
        Self {
            user_id: value.user_id,
            group_ids: value.group_ids,
        }
    }
}

impl SetUserGroupsParam {
    fn to_avs(self) -> Vec<user_group::ActiveModel> {
        self.group_ids
            .into_iter()
            .map(|group_id: String| user_group::ActiveModel {
                user_id: to_av!(self.user_id.clone()),
                group_id: to_av!(group_id),
                status: to_av!(YesNo::Yes.val()),
                created: utc_now_av!(),
                updated: to_av!(None::<i64>),
                ..Default::default()
            })
            .collect()
    }
}
pub struct UserGroupManageService;

impl UserGroupManageService {
    pub async fn set_user_groups(db: &DbConn, param: SetUserGroupsParam) -> Result<(), DbErr> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                UserGroup::delete_many()
                    .filter(user_group::Column::UserId.eq(&param.user_id))
                    .exec(txn)
                    .await?;
                let groups = param.to_avs();
                if groups.len() > 0 {
                    UserGroup::insert_many(groups).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
        .map_err(|err| match err {
            TransactionError::Connection(db_err) => db_err,
            TransactionError::Transaction(e) => e,
        })
    }
    pub async fn get_user_groups(db: &DbConn, user_id: &str) -> Result<Vec<String>, DbErr> {
        UserGroup::find()
            .select_only()
            .column(user_group::Column::GroupId)
            .filter(user_group::Column::UserId.eq(user_id))
            .filter(user_group::Column::Status.eq(YesNo::Yes.val()))
            .into_tuple()
            .all(db)
            .await
    }
}
