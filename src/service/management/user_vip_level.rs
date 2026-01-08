use crate::entity::{prelude::UserVipLevel, user_vip_level};
use chrono::{DateTime, Days, Months, TimeZone, Utc};
use sea_orm::*;
use tower::{
    common::{dict::VipLevel, to_av, utc_now_av, utc_now_av_opt, YesNo},
    management_model::SaveUserVipLevelReq,
    server_lib::utc_secs,
};
pub struct SaveVipLevelParam {
    user_id: String,
    vip_level: VipLevel,
    months: u32,
    days: u64,
}
impl SaveVipLevelParam {
    pub fn expired(&self, start: DateTime<Utc>) -> i64 {
        let mut ret = start;
        if self.days > 0 {
            ret = ret.checked_add_days(Days::new(self.days)).unwrap();
        }
        if self.months > 0 {
            ret = ret.checked_add_months(Months::new(self.months)).unwrap();
        }
        ret.timestamp()
    }
}
impl From<SaveUserVipLevelReq> for SaveVipLevelParam {
    fn from(value: SaveUserVipLevelReq) -> Self {
        let SaveUserVipLevelReq {
            user_id,
            vip_level,
            months,
            days,
        } = value;
        Self {
            user_id,
            vip_level,
            months,
            days,
        }
    }
}

pub struct UserVipLevelManageService;

impl UserVipLevelManageService {
    pub async fn save_vip_level(db: &DbConn, param: SaveVipLevelParam) -> Result<(), DbErr> {
        let now = Utc::now();
        let exist = UserVipLevel::find_by_id(param.user_id.clone())
            .one(db)
            .await?;
        match exist {
            Some(exist) => {
                let start = if now.timestamp() > exist.expried {
                    now
                } else {
                    Utc.timestamp_opt(exist.expried, 0).unwrap()
                };

                let mut update: user_vip_level::ActiveModel = exist.into();

                update.level_code = to_av!(param.vip_level.to_string());
                update.expried = to_av!(param.expired(start));
                update.status = to_av!(YesNo::Yes.val());
                update.updated = utc_now_av_opt!();

                update.update(db).await?;
                Ok(())
            }
            None => {
                let insert = user_vip_level::ActiveModel {
                    user_id: to_av!(param.user_id.clone()),
                    level_code: to_av!(param.vip_level.to_string()),
                    status: to_av!(YesNo::Yes.val()),
                    expried: to_av!(param.expired(now)),
                    created: utc_now_av!(),
                    updated: to_av!(None::<i64>),
                };
                insert.insert(db).await?;
                Ok(())
            }
        }
    }

    pub async fn get_vip_level_by_user(
        db: &DbConn,
        user_id: &str,
    ) -> Result<Option<user_vip_level::Model>, DbErr> {
        UserVipLevel::find_by_id(user_id)
            .filter(
                Condition::all()
                    .add(user_vip_level::Column::Status.eq(YesNo::Yes.val()))
                    .add(user_vip_level::Column::Expried.gte(utc_secs())),
            )
            .one(db)
            .await
    }
}
