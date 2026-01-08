use sea_orm::*;
use serde::{Deserialize, Serialize};
use tower::common::{to_av, user::AddFeedBackReq, utc_now_av};

use crate::entity::user_feed_back;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFeedBackParam {
    pub user_id: String,
    pub app_id: String,
    pub module: String,
    pub content: String,
    pub contact: String,
}

impl AddFeedBackParam {
    pub fn from(req: AddFeedBackReq, user_id: &str) -> Self {
        let AddFeedBackReq {
            app_id,
            module,
            content,
            contact,
        } = req;
        Self {
            user_id: user_id.to_string(),
            app_id,
            module,
            content,
            contact,
        }
    }
}

impl IntoActiveModel<user_feed_back::ActiveModel> for AddFeedBackParam {
    fn into_active_model(self) -> user_feed_back::ActiveModel {
        user_feed_back::ActiveModel {
            user_id: to_av!(self.user_id),
            app_id: to_av!(self.app_id),
            module: to_av!(self.module),
            content: to_av!(self.content),
            contact: to_av!(self.contact),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UserFeedBackService;

impl UserFeedBackService {
    pub async fn add_feed_back(db: &DbConn, param: AddFeedBackParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.id)
    }
}
