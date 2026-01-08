use sea_orm::*;
use tower::common::YesNo;

use crate::entity::prelude::UserApp;
use crate::entity::user_app;

pub struct UserAppService;

impl UserAppService {
    pub async fn get_apps(db: &DbConn, user_id: &str) -> Result<Vec<user_app::Model>, DbErr> {
        UserApp::find()
            .filter(user_app::Column::UserId.eq(user_id))
            .filter(user_app::Column::Status.eq(YesNo::Yes.val()))
            .all(db)
            .await
    }
}
