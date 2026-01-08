use actix_web::web;

pub mod config;
pub mod dict;
pub mod dict_item;
pub mod group;
pub mod role;
pub(crate) mod sync_util;
pub mod user;
pub mod user_app;
pub mod user_group;
pub mod user_role;
pub mod user_vip_level;

pub fn config(cfg: &mut web::ServiceConfig) {
    config::config(cfg);
    dict::config(cfg);
    dict_item::config(cfg);
    user::config(cfg);
    group::config(cfg);
    role::config(cfg);
    user_role::config(cfg);
    user_group::config(cfg);
    user_vip_level::config(cfg);
    user_app::config(cfg);
}
