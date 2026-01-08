use actix_web::web;

pub mod auth;
pub mod user_feed_back;

pub fn config(cfg: &mut web::ServiceConfig) {
    auth::config(cfg);
    user_feed_back::config(cfg);
}
