use actix_web::web;

pub mod config;
pub mod dict;
pub mod management;
pub mod public;
pub mod social;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    social::config(cfg);
    config::config(cfg);
    public::config(cfg);
    dict::config(cfg);
    user::config(cfg);
    management::config(cfg);
}
