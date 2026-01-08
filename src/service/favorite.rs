use chrono::Utc;
use sea_orm::*;
use tower::common::social::FavoriteReq;

use crate::{
    entity::{favorite, prelude::*, social_report},
    service::SocialReportService,
};

#[derive(Debug)]
pub struct FavoriteParam {
    pub user_id: String,
    pub target_id: String,
    pub target_type: i16,
}
impl FavoriteParam {
    pub fn of(user_id: &str, value: &FavoriteReq) -> Self {
        Self {
            user_id: user_id.to_owned(),
            target_id: value.target_id.to_owned(),
            target_type: value.target_type.to_owned(),
        }
    }
}
impl IntoActiveModel<favorite::ActiveModel> for FavoriteParam {
    fn into_active_model(self) -> favorite::ActiveModel {
        favorite::ActiveModel {
            user_id: self.user_id.into_active_value(),
            target_id: self.target_id.into_active_value(),
            target_type: self.target_type.into_active_value(),
            created: Utc::now().timestamp().into_active_value(),
            updated: None::<i64>.into_active_value(),
            ..Default::default()
        }
    }
}

pub struct FavoriteService;
impl FavoriteService {
    ///若不存在，则保存
    pub async fn favorite(db: &DbConn, param: FavoriteParam) -> Result<(), DbErr> {
        let exist = Favorite::find()
            .filter(
                Condition::all()
                    .add(favorite::Column::UserId.eq(&param.user_id))
                    .add(favorite::Column::TargetId.eq(&param.target_id))
                    .add(favorite::Column::TargetType.eq(param.target_type)),
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
            social_report::Column::FavoriteCount,
            1,
        )
        .await?;
        Ok(())
    }

    pub async fn unfavorite(db: &DbConn, param: FavoriteParam) -> Result<(), DbErr> {
        let ret = Favorite::delete_many()
            .filter(
                Condition::all()
                    .add(favorite::Column::UserId.eq(&param.user_id))
                    .add(favorite::Column::TargetId.eq(&param.target_id))
                    .add(favorite::Column::TargetType.eq(param.target_type)),
            )
            .exec(db)
            .await?;

        if ret.rows_affected > 0 {
            SocialReportService::update_count(
                db,
                &param.target_id,
                param.target_type,
                social_report::Column::FavoriteCount,
                -1,
            )
            .await?;
        }
        Ok(())
    }

    pub async fn get_my_favorites(
        db: &DbConn,
        user_id: &str,
        target_type: i16,
    ) -> Result<Vec<String>, DbErr> {
        Favorite::find()
            .select_only()
            .column(favorite::Column::TargetId)
            .filter(favorite::Column::UserId.eq(user_id))
            .filter(favorite::Column::TargetType.eq(target_type))
            .into_tuple()
            .all(db)
            .await
    }
}
