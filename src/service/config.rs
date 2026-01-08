use sea_orm::*;
use tower::common::YesNo;

use crate::entity::{config, prelude::Config};

pub struct ConfigService;

impl ConfigService {
    pub async fn list(
        db: &DbConn,
        app_id: &str,
        public: Option<YesNo>,
    ) -> Result<Vec<config::Model>, DbErr> {
        Config::find()
            .filter(config::Column::AppId.eq(app_id))
            .filter(config::Column::Status.eq(YesNo::Yes.val()))
            .apply_if(public, |query, val| {
                query.filter(config::Column::Public.eq(Into::<i16>::into(val)))
            })
            .all(db)
            .await
    }

    pub async fn get_by_key(
        db: &DbConn,
        app_id: &str,
        config_key: &str,
        public: YesNo,
    ) -> Result<Option<config::Model>, DbErr> {
        Config::find()
            .filter(config::Column::AppId.eq(app_id))
            .filter(config::Column::VarName.eq(config_key))
            .filter(config::Column::Status.eq(YesNo::Yes.val()))
            .filter(config::Column::Public.eq(public.val()))
            .one(db)
            .await
    }

    async fn batch_save(db: &DbConn, configs: Vec<config::ActiveModel>) -> Result<i32, DbErr> {
        let ret = Config::insert_many(configs).exec(db).await?;
        Ok(ret.last_insert_id)
    }
}
#[cfg(test)]
mod tests {
    use sea_orm::*;
    use tower::{
        common::App,
        common::{to_av, utc_now_av},
    };

    use crate::{
        entity::config,
        service::{tests::init_state, ConfigService},
        tests::init_log,
    };

    #[test]
    fn it_works() {}

    #[tokio::test]
    async fn insert_config() {
        init_log();
        let state = init_state().await;
        let app = App::TowerTauri;

        let vars = vec![("TowerServer", "http://127.0.0.1:4000")];

        insert_public_config(&state.conn, &app, vars).await.unwrap();

        let vars = vec![
            ("ListenAddr", "127.0.0.1:4002"),
            ("WorkerNum", "1"),
            ("AllowOrigin", "*"),
            ("DevMode", "true"),
        ];

        insert_private_config(&state.conn, &app, vars)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn insert_config_tmp() {
        init_log();
        let state = init_state().await;
        let app = App::TowerTauri;

        // let vars = vec![("TowerAssitantServer", "http://127.0.0.1:4002")];

        let vars = vec![
            ("TowerTauri", "http://tauri.localhost"),
            ("TowerManagementWebsite", "http://manager.5-tower.online"),
        ];

        insert_public_config(&state.conn, &app, vars).await.unwrap();
        // insert_private_config(&state.conn, &app, vars)
        //     .await
        // .unwrap();
    }

    async fn insert_public_config(
        db: &DbConn,
        app: &App,
        vars: Vec<(&str, &str)>,
    ) -> Result<i32, DbErr> {
        let configs: Vec<config::ActiveModel> = vars
            .iter()
            .map(|(name, value)| {
                let name = (*name).to_string();
                let value = (*value).to_string();
                config::ActiveModel {
                    app_id: to_av!(app.to_string()),
                    app_version: to_av!(None::<String>),
                    var_name: to_av!(name),
                    var_value: to_av!(value.to_string()),
                    public: to_av!(0),
                    remark: to_av!(None::<String>),
                    status: to_av!(0),
                    created: utc_now_av!(),
                    updated: to_av!(None::<i64>),
                    ..Default::default()
                }
            })
            .collect();

        ConfigService::batch_save(db, configs).await
    }

    async fn insert_private_config(
        db: &DbConn,
        app: &App,
        vars: Vec<(&str, &str)>,
    ) -> Result<i32, DbErr> {
        let configs: Vec<config::ActiveModel> = vars
            .iter()
            .map(|(name, value)| {
                let name = (*name).to_string();
                let value = (*value).to_string();
                config::ActiveModel {
                    app_id: to_av!(app.to_string()),
                    app_version: to_av!(None::<String>),
                    var_name: to_av!(name),
                    var_value: to_av!(value.to_string()),
                    public: to_av!(1),
                    remark: to_av!(None::<String>),
                    status: to_av!(0),
                    created: utc_now_av!(),
                    updated: to_av!(None::<i64>),
                    ..Default::default()
                }
            })
            .collect();

        ConfigService::batch_save(db, configs).await
    }
}
