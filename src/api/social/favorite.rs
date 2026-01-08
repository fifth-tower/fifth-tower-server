use actix_web::{delete, get, post, web, Error, HttpResponse};
use tower::common::{social::FavoriteReq, social::MyFavoritesReq, Urlable};
use tower::server_lib::TowerPrincipal;
use tower::{common::TowerResource, server_lib::to_resp};

use crate::service::{FavoriteParam, FavoriteService};
use crate::StateEx;

/// 收藏
#[post("")]
pub async fn favorite(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<FavoriteReq>,
) -> Result<HttpResponse, Error> {
    let ret =
        FavoriteService::favorite(&config.conn, FavoriteParam::of(&principal.user_id, &req)).await;

    to_resp(ret)
}
/// 取消收藏
#[delete("/many")]
pub async fn unfavorite(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Json<FavoriteReq>,
) -> Result<HttpResponse, Error> {
    let ret =
        FavoriteService::unfavorite(&config.conn, FavoriteParam::of(&principal.user_id, &req))
            .await;

    to_resp(ret)
}

#[get("/list/my")]
pub async fn get_my_favorites(
    principal: TowerPrincipal,
    config: StateEx,
    req: web::Query<MyFavoritesReq>,
) -> Result<HttpResponse, Error> {
    let ret =
        FavoriteService::get_my_favorites(&config.conn, &principal.user_id, req.target_type).await;

    to_resp(ret)
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Favorite.scope())
            .service(favorite)
            .service(unfavorite)
            .service(get_my_favorites),
    );
}
