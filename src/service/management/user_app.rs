use sea_orm::*;
use tower::common::{to_av, utc_now_av, YesNo};
use tower::management_model::SetUserAppsReq;

use crate::entity::prelude::UserApp;
use crate::entity::user_app;

#[derive(Debug)]
pub struct SetUserAppsParam {
    pub user_id: String,
    pub app_ids: Vec<String>,
}

impl From<SetUserAppsReq> for SetUserAppsParam {
    fn from(value: SetUserAppsReq) -> Self {
        Self {
            user_id: value.user_id,
            app_ids: value.app_ids,
        }
    }
}

impl SetUserAppsParam {
    fn to_avs(self) -> Vec<user_app::ActiveModel> {
        self.app_ids
            .into_iter()
            .map(|app_id: String| user_app::ActiveModel {
                user_id: to_av!(self.user_id.clone()),
                app_id: to_av!(app_id),
                status: to_av!(YesNo::Yes.val()),
                created: utc_now_av!(),
                updated: to_av!(None::<i64>),
                ..Default::default()
            })
            .collect()
    }
}
pub struct UserAppManageService;

impl UserAppManageService {
    pub async fn set_user_apps(db: &DbConn, param: SetUserAppsParam) -> Result<(), DbErr> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                UserApp::delete_many()
                    .filter(user_app::Column::UserId.eq(&param.user_id))
                    .exec(txn)
                    .await?;
                let apps = param.to_avs();
                if apps.len() > 0 {
                    UserApp::insert_many(apps).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
        .map_err(|err| match err {
            TransactionError::Connection(db_err) => db_err,
            TransactionError::Transaction(e) => e,
        })
    }
    pub async fn get_user_apps(db: &DbConn, user_id: &str) -> Result<Vec<String>, DbErr> {
        UserApp::find()
            .select_only()
            .column(user_app::Column::AppId)
            .filter(user_app::Column::UserId.eq(user_id))
            .filter(user_app::Column::Status.eq(YesNo::Yes.val()))
            .into_tuple()
            .all(db)
            .await
    }
}
