use sea_orm::*;
use tower::common::{social::CommentReq, to_av, utc_now_av};

use crate::{
    entity::{commentt, social_report},
    service::SocialReportService,
};

#[derive(Debug)]
pub struct CommentParam {
    pub user_id: String,
    pub target_id: String,
    pub target_type: i16,
    pub content: String,
    pub parent_id: Option<String>,
}
impl CommentParam {
    pub fn of(user_id: &str, value: &CommentReq) -> Self {
        Self {
            user_id: user_id.to_owned(),
            target_id: value.target_id.to_owned(),
            target_type: value.target_type.to_owned(),
            content: value.content.to_owned(),
            parent_id: value.parent_id.to_owned(),
        }
    }
}
impl IntoActiveModel<commentt::ActiveModel> for CommentParam {
    fn into_active_model(self) -> commentt::ActiveModel {
        commentt::ActiveModel {
            user_id: to_av!(self.user_id),
            target_id: to_av!(self.target_id),
            target_type: to_av!(self.target_type),
            content: to_av!(self.content),
            parent_id: to_av!(self.parent_id),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}

pub struct CommentService;
impl CommentService {
    pub async fn comment(db: &DbConn, param: CommentParam) -> Result<(), DbErr> {
        let target_id = param.target_id.clone();
        let target_type = param.target_type;

        let _ = param.into_active_model().insert(db).await?;

        SocialReportService::update_count(
            db,
            &target_id,
            target_type,
            social_report::Column::CommentCount,
            1,
        )
        .await?;
        Ok(())
    }
}
