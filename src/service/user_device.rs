use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};

use crate::entity::prelude::UserDevice;
use crate::entity::user_device;

#[derive(Debug)]
pub struct AddDeviceParam {
    pub user_id: String,
    pub device_code: String,
    pub ip: Option<String>,
}

impl IntoActiveModel<user_device::ActiveModel> for AddDeviceParam {
    fn into_active_model(self) -> user_device::ActiveModel {
        user_device::ActiveModel {
            user_id: to_av!(self.user_id),
            device_code: to_av!(self.device_code),
            ip: to_av!(self.ip),
            status: to_av!(YesNo::Yes.val()),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UserDeviceService;

impl UserDeviceService {
    pub async fn add_device(db: &DbConn, param: AddDeviceParam) -> Result<i32, DbErr> {
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.id)
    }

    pub async fn get_latest_device(
        db: &DbConn,
        user_id: &str,
    ) -> Result<Option<user_device::Model>, DbErr> {
        let device = UserDevice::find()
            .filter(user_device::Column::UserId.eq(user_id))
            .order_by_desc(user_device::Column::Created)
            .limit(1)
            .one(db)
            .await?;
        Ok(device)
    }
}
