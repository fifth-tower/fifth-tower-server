use crate::config::get_env;
use crate::config::AppState;
use actix_web::web;
use actix_web::Error;
use tower::common::DEV_MODE;
use tower::server_lib::run_server;

mod api;
mod config;
mod entity;
mod service;

pub type StateEx = web::Data<AppState>;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let vars = get_env(DEV_MODE);

    run_server(&vars, async { AppState::new(&vars).await }, api::config).await
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn init_log() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    }
}
