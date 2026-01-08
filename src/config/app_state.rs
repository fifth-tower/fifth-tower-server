use std::{collections::HashMap, sync::Mutex, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tower::{
    common::{dict::DictData, wrap_config_impl, App, ConfigGetter},
    config_model::Config,
    server_lib::EnvKey,
};

use crate::api::{config::load_configs, dict::get_dicts_data};

/// Application config
#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub config: Mutex<Config>,
    pub dicts: Mutex<DictData>,
}

impl AppState {
    pub async fn new(vars: &HashMap<EnvKey, String>) -> Self {
        // you will most probably do some async stuff here like open a DB pool ...
        let mut opt = ConnectOptions::new(vars.get(&EnvKey::DatabaseUrl).unwrap());
        opt.max_connections(10)
            .min_connections(10)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            // .sqlx_logging_level(log::LevelFilter::INFO)
            .set_schema_search_path("public");

        let conn = Database::connect(opt).await.unwrap();
        let config = load_configs(&conn, &App::TowerServer).await.unwrap();
        let dicts = get_dicts_data(&conn).await.unwrap();
        Self {
            conn,
            config: Mutex::new(config),
            dicts: Mutex::new(dicts),
        }
    }

    pub async fn refresh_config(&self) -> Result<(), DbErr> {
        let config = load_configs(&self.conn, &App::TowerServer).await?;
        let mut config_state = self.config.lock().unwrap();
        config_state.set_data(config.data());

        Ok(())
    }

    pub async fn refresh_dict(&self) -> Result<(), DbErr> {
        let dicts = get_dicts_data(&self.conn).await?;
        let mut dicts_state = self.dicts.lock().unwrap();
        dicts_state.clone_from(dicts);

        Ok(())
    }

    pub fn config(&self) -> Config {
        let config_state = self.config.lock().unwrap();
        config_state.clone()
    }
}
wrap_config_impl! {AppState}
