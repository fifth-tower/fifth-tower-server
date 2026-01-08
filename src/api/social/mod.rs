use actix_web::web;

pub mod comment;
pub mod favorite;
pub mod follow;
pub mod like;
pub mod report;
pub mod score;

pub fn config(cfg: &mut web::ServiceConfig) {
    favorite::config(cfg);
    like::config(cfg);
    score::config(cfg);
    comment::config(cfg);
    follow::config(cfg);
    report::config(cfg);
}
