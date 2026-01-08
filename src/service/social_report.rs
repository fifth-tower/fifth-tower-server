use sea_orm::{prelude::Expr, *};
use tower::common::{to_av, utc_now_av};
use tower::server_lib::utc_secs;

use crate::entity::prelude::SocialReport;
use crate::entity::social_report::{self};

pub struct SocialReportService;
impl SocialReportService {
    pub async fn insert(db: &DbConn, target_id: &str, target_type: i16) -> Result<(), DbErr> {
        let model = social_report::ActiveModel {
            target_id: to_av!(target_id.to_string()),
            target_type: to_av!(target_type),
            like_count: to_av!(0),
            follow_count: to_av!(0),
            favorite_count: to_av!(0),
            comment_count: to_av!(0),
            score_count: to_av!(0),
            score_total: to_av!(0),
            created: utc_now_av!(),
            updated: None::<i64>.into_active_value(),
            ..Default::default()
        };
        let _ = model.insert(db).await?;
        Ok(())
    }

    pub async fn update_total_and_count<T>(
        db: &DbConn,
        target_id: &str,
        target_type: i16,
        count_column: social_report::Column,
        total_column: social_report::Column,
        add_value: T,
    ) -> Result<(), DbErr>
    where
        T: Into<i64>,
    {
        let add_value = Into::<i64>::into(add_value);
        if add_value == 0 {
            return Ok(());
        }
        SocialReport::update_many()
            .col_expr(
                count_column,
                if add_value > 0 {
                    Expr::col(count_column).add(1)
                } else {
                    Expr::col(count_column).sub(1)
                },
            )
            .col_expr(
                total_column,
                if add_value > 0 {
                    Expr::col(total_column).add(add_value)
                } else {
                    Expr::col(total_column).sub(add_value.abs())
                },
            )
            .col_expr(social_report::Column::Updated, Expr::value(utc_secs()))
            .filter(social_report::Column::TargetId.eq(target_id))
            .filter(social_report::Column::TargetType.eq(target_type))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn update_count<T>(
        db: &DbConn,
        target_id: &str,
        target_type: i16,
        count_column: social_report::Column,
        add_value: T,
    ) -> Result<(), DbErr>
    where
        T: Into<i64>,
    {
        let add_value = Into::<i64>::into(add_value);
        if add_value == 0 {
            return Ok(());
        }
        SocialReport::update_many()
            .col_expr(
                count_column,
                if add_value > 0 {
                    Expr::col(count_column).add(add_value)
                } else {
                    Expr::col(count_column).sub(add_value.abs())
                },
            )
            .col_expr(social_report::Column::Updated, Expr::value(utc_secs()))
            .filter(social_report::Column::TargetId.eq(target_id))
            .filter(social_report::Column::TargetType.eq(target_type))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn update_total<T>(
        db: &DbConn,
        target_id: &str,
        target_type: i16,
        total_column: social_report::Column,
        add_value: T,
    ) -> Result<(), DbErr>
    where
        T: Into<i64>,
    {
        let add_value = Into::<i64>::into(add_value);
        if add_value == 0 {
            return Ok(());
        }
        SocialReport::update_many()
            .col_expr(
                total_column,
                if add_value > 0 {
                    Expr::col(total_column).add(add_value)
                } else {
                    Expr::col(total_column).sub(add_value.abs())
                },
            )
            .col_expr(social_report::Column::Updated, Expr::value(utc_secs()))
            .filter(social_report::Column::TargetId.eq(target_id))
            .filter(social_report::Column::TargetType.eq(target_type))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_report_by_ids(
        db: &DbConn,
        target_ids: Vec<String>,
        target_type: i16,
    ) -> Result<Vec<social_report::Model>, DbErr> {
        SocialReport::find()
            .filter(social_report::Column::TargetId.is_in(target_ids))
            .filter(social_report::Column::TargetType.eq(target_type))
            .all(db)
            .await
    }
}
