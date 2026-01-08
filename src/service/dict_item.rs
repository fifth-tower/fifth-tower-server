use sea_orm::*;
use tower::common::YesNo;

use crate::entity::dict_item;
use crate::entity::prelude::*;

pub struct DictItemService;

impl DictItemService {
    pub async fn get_items_by_code<T>(db: &DbConn, code: T) -> Result<Vec<dict_item::Model>, DbErr>
    where
        T: Into<String>,
    {
        DictItem::find()
            .filter(
                Condition::all()
                    .add(dict_item::Column::DictCode.eq(&code.into()))
                    .add(dict_item::Column::Status.eq(YesNo::Yes.val())),
            )
            .all(db)
            .await
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<dict_item::Model>, DbErr> {
        DictItem::find()
            .filter(dict_item::Column::Status.eq(YesNo::Yes.val()))
            .all(db)
            .await
    }
    pub async fn batch_save(db: &DbConn, dicts: Vec<dict_item::ActiveModel>) -> Result<i32, DbErr> {
        let ret = DictItem::insert_many(dicts).exec(db).await?;
        Ok(ret.last_insert_id)
    }
}
