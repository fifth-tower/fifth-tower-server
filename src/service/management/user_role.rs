use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};
use tower::management_model::SetUserRolesReq;

use crate::entity::prelude::UserRole;
use crate::entity::user_role;

#[derive(Debug)]
pub struct SetUserRolesParam {
    pub user_id: String,
    pub role_ids: Vec<String>,
}

impl From<SetUserRolesReq> for SetUserRolesParam {
    fn from(value: SetUserRolesReq) -> Self {
        Self {
            user_id: value.user_id,
            role_ids: value.role_ids,
        }
    }
}

impl SetUserRolesParam {
    fn to_avs(self) -> Vec<user_role::ActiveModel> {
        self.role_ids
            .into_iter()
            .map(|role_id: String| user_role::ActiveModel {
                user_id: to_av!(self.user_id.clone()),
                role_id: to_av!(role_id),
                status: to_av!(YesNo::Yes.val()),
                created: utc_now_av!(),
                updated: to_av!(None::<i64>),
                ..Default::default()
            })
            .collect()
    }
}
pub struct UserRoleManageService;

impl UserRoleManageService {
    pub async fn set_user_roles(db: &DbConn, param: SetUserRolesParam) -> Result<(), DbErr> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                UserRole::delete_many()
                    .filter(user_role::Column::UserId.eq(&param.user_id))
                    .exec(txn)
                    .await?;
                let roles = param.to_avs();
                if roles.len() > 0 {
                    UserRole::insert_many(roles).exec(txn).await?;
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
    pub async fn get_user_roles(db: &DbConn, user_id: &str) -> Result<Vec<String>, DbErr> {
        UserRole::find()
            .select_only()
            .column(user_role::Column::RoleId)
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::Status.eq(YesNo::Yes.val()))
            .into_tuple()
            .all(db)
            .await
    }
}
