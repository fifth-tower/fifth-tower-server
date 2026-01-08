use sea_orm::*;
use tower::common::{social::ScoreReq, to_av, utc_now_av};

use crate::{
    entity::{prelude::*, score, social_report},
    service::SocialReportService,
};

#[derive(Debug)]
pub struct ScoreParam {
    pub user_id: String,
    pub target_id: String,
    pub target_type: i16,
    pub score: i64,
    pub comment: Option<String>,
}
impl ScoreParam {
    pub fn of(user_id: &str, value: &ScoreReq) -> Self {
        Self {
            user_id: user_id.to_owned(),
            target_id: value.target_id.to_owned(),
            target_type: value.target_type.to_owned(),
            score: value.score.to_owned(),
            comment: value.comment.to_owned(),
        }
    }
}
impl IntoActiveModel<score::ActiveModel> for ScoreParam {
    fn into_active_model(self) -> score::ActiveModel {
        score::ActiveModel {
            user_id: to_av!(self.user_id),
            target_id: to_av!(self.target_id),
            target_type: to_av!(self.target_type),
            score: to_av!(self.score),
            comment: to_av!(self.comment),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}

pub struct ScoreService;
impl ScoreService {
    pub async fn score(db: &DbConn, param: ScoreParam) -> Result<(), DbErr> {
        let exist = Score::find()
            .filter(
                Condition::all()
                    .add(score::Column::UserId.eq(&param.user_id))
                    .add(score::Column::TargetId.eq(&param.target_id))
                    .add(score::Column::TargetType.eq(param.target_type)),
            )
            .one(db)
            .await?;
        if exist.is_some() {
            let exist = exist.unwrap();
            let add_value = param.score - exist.score;

            let mut model: score::ActiveModel = exist.into();
            model.score = to_av!(param.score);
            model.comment = to_av!(param.comment);

            model.update(db).await?;

            SocialReportService::update_total(
                db,
                &param.target_id,
                param.target_type,
                social_report::Column::ScoreTotal,
                add_value,
            )
            .await?;
            return Ok(());
        }
        let target_id = param.target_id.clone();
        let target_type = param.target_type;
        let score = param.score;

        let _ = param.into_active_model().insert(db).await?;

        SocialReportService::update_total_and_count(
            db,
            &target_id,
            target_type,
            social_report::Column::ScoreCount,
            social_report::Column::ScoreTotal,
            score,
        )
        .await?;
        Ok(())
    }

    pub async fn get_my_scores(
        db: &DbConn,
        user_id: &str,
        target_type: i16,
    ) -> Result<Vec<(String, i64)>, DbErr> {
        Score::find()
            .select_only()
            .column(score::Column::TargetId)
            .column(score::Column::Score)
            .filter(score::Column::UserId.eq(user_id))
            .filter(score::Column::TargetType.eq(target_type))
            .into_tuple()
            .all(db)
            .await
    }
}
