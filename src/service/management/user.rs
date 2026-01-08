use sea_orm::prelude::Expr;
use sea_orm::*;
use tower::common::{Page, YesNo};
use tower::management_model::UserListReq;

use crate::entity::prelude::User;
use crate::entity::user;
use crate::service::DEFAULT_PASSWORD;

pub struct UserManageService;

pub struct GetPagedUsersParam {
    username: String,
    nickname: String,
    status: Option<YesNo>,
}

impl GetPagedUsersParam {
    pub fn from(req: UserListReq) -> Self {
        let UserListReq {
            username,
            nickname,
            status,
        } = req;
        Self {
            username,
            nickname,
            status,
        }
    }
}
impl UserManageService {
    pub async fn get_paged_users(
        db: &DbConn,
        param: Page<GetPagedUsersParam>,
    ) -> Result<Page<Vec<user::Model>>, DbErr> {
        let user_pages = User::find()
            .apply_if(
                (param.username.len() > 0).then_some(param.username.clone()),
                |query, v| query.filter(user::Column::Username.starts_with(v)),
            )
            .apply_if(
                (param.nickname.len() > 0).then_some(param.nickname.clone()),
                |query, v| query.filter(user::Column::Nickname.like(v)),
            )
            .apply_if(param.status, |query, v| {
                query.filter(user::Column::Status.eq(v.val()))
            })
            .order_by_desc(user::Column::Created)
            .paginate(db, param.page_size);

        let data = user_pages.fetch_page(param.page).await?;
        let mut ret_page = Page::new(param.page, param.page_size, data);
        if param.page == 0 {
            let total = user_pages.num_items().await?;
            ret_page.set_total(total);
        }
        Ok(ret_page)
    }

    pub async fn set_status(db: &DbConn, user_id: &str, status: i16) -> Result<(), DbErr> {
        User::update_many()
            .col_expr(user::Column::Status, Expr::value(status))
            .filter(user::Column::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn reset_password(db: &DbConn, user_id: &str) -> Result<(), DbErr> {
        User::update_many()
            .col_expr(user::Column::Password, Expr::value(DEFAULT_PASSWORD))
            .filter(user::Column::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(())
    }
}
