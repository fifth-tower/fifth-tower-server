use sea_orm::*;
use tower::common::{random_id, to_av, user::RegisterReq, utc_now_av, YesNo};
use tower::management_model::AddUserReq;

use crate::entity::prelude::User;
use crate::entity::user::{self};

pub const DEFAULT_PASSWORD: &str = "123456";

pub struct AddUserParam {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub avatar: i16,
}

impl From<RegisterReq> for AddUserParam {
    fn from(register: RegisterReq) -> Self {
        Self {
            username: register.username.clone(),
            nickname: register.nickname.clone(),
            password: register.password.clone(),
            avatar: register.avatar,
        }
    }
}
impl From<AddUserReq> for AddUserParam {
    fn from(value: AddUserReq) -> Self {
        Self {
            username: value.username.clone(),
            nickname: value.nickname.clone(),
            password: DEFAULT_PASSWORD.to_string(),
            avatar: value.avatar,
        }
    }
}

impl IntoActiveModel<user::ActiveModel> for AddUserParam {
    fn into_active_model(self) -> user::ActiveModel {
        user::ActiveModel {
            user_id: to_av!(random_id(8)),
            username: to_av!(self.username),
            nickname: to_av!(self.nickname),
            password: to_av!(self.password),
            avatar: to_av!(self.avatar),
            status: to_av!(YesNo::Yes.val()),
            created: utc_now_av!(),
            updated: to_av!(None::<i64>),
            ..Default::default()
        }
    }
}
pub struct UserService;

impl UserService {
    pub async fn add_user(db: &DbConn, param: AddUserParam) -> Result<String, DbErr> {
        let exist = User::find()
            .filter(user::Column::Username.eq(&param.username))
            .all(db)
            .await?;
        if exist.len() > 0 {
            return Err(DbErr::Custom("帐号已存在。".to_string()));
        }
        let exist = User::find()
            .filter(user::Column::Nickname.eq(&param.nickname))
            .all(db)
            .await?;
        if exist.len() > 0 {
            return Err(DbErr::Custom("昵称已存在。".to_string()));
        }
        let ret = param.into_active_model().insert(db).await?;
        Ok(ret.user_id)
    }

    pub async fn get_user_by_username(
        db: &DbConn,
        user_name: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Username.eq(user_name))
            .one(db)
            .await
    }

    pub async fn get_user_by_id(db: &DbConn, user_id: &str) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    pub async fn change_password(
        db: &DbConn,
        user_id: String,
        password: String,
    ) -> Result<(), DbErr> {
        let user = user::ActiveModel {
            password: to_av!(password),
            user_id: ActiveValue::Unchanged(user_id),
            ..Default::default()
        };
        user.update(db).await?;
        Ok(())
    }
}
