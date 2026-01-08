use sea_orm::*;
use tower::common::YesNo;

use crate::entity::dict;
use crate::entity::prelude::*;

pub struct DictService;

impl DictService {
    pub async fn get_dicts_by_code<T>(db: &DbConn, code: T) -> Result<Vec<dict::Model>, DbErr>
    where
        T: Into<String>,
    {
        Dict::find()
            .filter(
                Condition::all()
                    .add(dict::Column::DictCode.eq(&code.into()))
                    .add(dict::Column::Status.eq(YesNo::Yes.val())),
            )
            .all(db)
            .await
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<dict::Model>, DbErr> {
        Dict::find()
            .filter(dict::Column::Status.eq(YesNo::Yes.val()))
            .all(db)
            .await
    }

    pub async fn batch_save(db: &DbConn, dicts: Vec<dict::ActiveModel>) -> Result<String, DbErr> {
        let ret = Dict::insert_many(dicts).exec(db).await?;
        Ok(ret.last_insert_id)
    }
}

#[cfg(test)]
mod tests {

    use sea_orm::*;
    use tower::common::{
        dict::{Dict, VipLevel, VipLevelItem},
        to_av, utc_now_av, YesNo,
    };

    use crate::{
        entity::{dict, dict_item},
        service::{tests::init_state, DictItemService, DictService},
        tests::init_log,
    };

    #[test]
    fn it_works() {}

    #[tokio::test]
    async fn insert_config_tmp() {
        init_log();
        let state = init_state().await;
    }

    #[tokio::test]
    async fn test_insert_viplevel_dicts() {
        init_log();
        let state = init_state().await;

        let dicts = vec![(Dict::VipLevel, None)];
        insert_viplevel_dicts(&state.conn, &dicts).await.unwrap();

        let dicts = vec![
            (VipLevel::Vip0, Some(Dict::VipLevel)),
            (VipLevel::Vip1, Some(Dict::VipLevel)),
            (VipLevel::Vip2, Some(Dict::VipLevel)),
            (VipLevel::Vip3, Some(Dict::VipLevel)),
            (VipLevel::Vip4, Some(Dict::VipLevel)),
            (VipLevel::Vip5, Some(Dict::VipLevel)),
            (VipLevel::Vip6, Some(Dict::VipLevel)),
            (VipLevel::Vip7, Some(Dict::VipLevel)),
            (VipLevel::Vip8, Some(Dict::VipLevel)),
            (VipLevel::Vip9, Some(Dict::VipLevel)),
        ];
        insert_viplevel_dicts(&state.conn, &dicts).await.unwrap();

        for (index, (vip, _)) in dicts.into_iter().enumerate() {
            let index = index + 1;
            let items = vec![
                (vip.clone(), VipLevelItem::InstanceNum, 1 * index),
                (vip.clone(), VipLevelItem::RecordFlowNum, 6 * index),
                (vip.clone(), VipLevelItem::RoomPersonNum, 5 * index),
                (vip.clone(), VipLevelItem::WishingNum, 8 * index),
            ];
            insert_viplevel_dict_items(&state.conn, items)
                .await
                .unwrap();
        }
    }
    async fn insert_viplevel_dicts<T>(
        db: &DbConn,
        dicts: &Vec<(T, Option<Dict>)>,
    ) -> Result<String, DbErr>
    where
        T: ToString,
    {
        let dicts: Vec<dict::ActiveModel> = dicts
            .iter()
            .map(|(code, parent)| {
                let code = code.to_string();
                let parent = parent.clone().map(|f| f.to_string());
                dict::ActiveModel {
                    dict_code: to_av!(code.clone()),
                    dict_name: to_av!(code),
                    parent_code: to_av!(parent),
                    remark: to_av!(None::<String>),
                    public: to_av!(YesNo::Yes.val()),
                    status: to_av!(YesNo::Yes.val()),
                    created: utc_now_av!(),
                    updated: to_av!(None::<i64>),
                }
            })
            .collect();

        DictService::batch_save(db, dicts).await
    }
    async fn insert_viplevel_dict_items<A, B, C>(
        db: &DbConn,
        items: Vec<(A, B, C)>,
    ) -> Result<i32, DbErr>
    where
        A: ToString,
        B: ToString,
        C: ToString,
    {
        let items: Vec<dict_item::ActiveModel> = items
            .iter()
            .map(|(code, name, value)| {
                let code = (*code).to_string();
                let name = (*name).to_string();
                let value = (*value).to_string();
                dict_item::ActiveModel {
                    dict_code: to_av!(code),
                    item_name: to_av!(name),
                    item_value: to_av!(value),
                    remark: to_av!(None::<String>),
                    public: to_av!(YesNo::Yes.val()),
                    status: to_av!(YesNo::Yes.val()),
                    created: utc_now_av!(),
                    updated: to_av!(None::<i64>),
                    ..Default::default()
                }
            })
            .collect();

        DictItemService::batch_save(db, items).await
    }
}
