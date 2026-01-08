use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};

use crate::entity::prelude::UserRole;
use crate::entity::user_role;

pub struct UserRoleService;

impl UserRoleService {
    pub async fn get_roles(db: &DbConn, user_id: &str) -> Result<Vec<user_role::Model>, DbErr> {
        UserRole::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::Status.eq(YesNo::Yes.val()))
            .all(db)
            .await
    }
}
