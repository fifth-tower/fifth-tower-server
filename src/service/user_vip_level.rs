use crate::entity::{prelude::UserVipLevel, user_vip_level};
use sea_orm::*;
use tower::{
    common::{dict::VipLevel, YesNo},
    server_lib::utc_secs,
};

pub struct UserVipLevelService;

impl UserVipLevelService {
    pub async fn get_vip_level_by_user(db: &DbConn, user_id: &str) -> Result<String, DbErr> {
        UserVipLevel::find_by_id(user_id)
            .filter(
                Condition::all()
                    .add(user_vip_level::Column::Status.eq(YesNo::Yes.val()))
                    .add(user_vip_level::Column::Expried.gte(utc_secs())),
            )
            .one(db)
            .await
            .map(|vip| {
                vip.map(|vip| vip.level_code)
                    .unwrap_or(VipLevel::Vip0.to_string())
            })
    }
}
