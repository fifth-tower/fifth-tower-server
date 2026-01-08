use chrono::Utc;
use sea_orm::*;
use tower::common::social::LikeReq;

use crate::{
    entity::{likee, prelude::*, social_report},
    service::SocialReportService,
};

#[derive(Debug)]
pub struct LikeParam {
    pub user_id: String,
    pub target_id: String,
    pub target_type: i16,
}
impl LikeParam {
    pub fn of(user_id: &str, value: &LikeReq) -> Self {
        Self {
            user_id: user_id.to_owned(),
            target_id: value.target_id.to_owned(),
            target_type: value.target_type.to_owned(),
        }
    }
}
impl IntoActiveModel<likee::ActiveModel> for LikeParam {
    fn into_active_model(self) -> likee::ActiveModel {
        likee::ActiveModel {
            user_id: self.user_id.into_active_value(),
            target_id: self.target_id.into_active_value(),
            target_type: self.target_type.into_active_value(),
            created: Utc::now().timestamp().into_active_value(),
            updated: None::<i64>.into_active_value(),
            ..Default::default()
        }
    }
}

pub struct LikeService;
impl LikeService {
    ///若不存在，则保存
    pub async fn like(db: &DbConn, param: LikeParam) -> Result<(), DbErr> {
        let exist = Likee::find()
            .filter(
                Condition::all()
                    .add(likee::Column::UserId.eq(&param.user_id))
                    .add(likee::Column::TargetId.eq(&param.target_id))
                    .add(likee::Column::TargetType.eq(param.target_type)),
            )
            .one(db)
            .await?;
        if exist.is_some() {
            return Ok(());
        }

        let target_id = param.target_id.clone();
        let target_type = param.target_type;
        let _ = param.into_active_model().insert(db).await?;

        SocialReportService::update_count(
            db,
            &target_id,
            target_type,
            social_report::Column::LikeCount,
            1,
        )
        .await?;
        Ok(())
    }

    pub async fn unlike(db: &DbConn, param: LikeParam) -> Result<(), DbErr> {
        let ret = Likee::delete_many()
            .filter(
                Condition::all()
                    .add(likee::Column::UserId.eq(&param.user_id))
                    .add(likee::Column::TargetId.eq(&param.target_id))
                    .add(likee::Column::TargetType.eq(param.target_type)),
            )
            .exec(db)
            .await?;

        if ret.rows_affected > 0 {
            SocialReportService::update_count(
                db,
                &param.target_id,
                param.target_type,
                social_report::Column::LikeCount,
                -1,
            )
            .await?;
        }
        Ok(())
    }
}
