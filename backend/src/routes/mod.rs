pub mod auth;
pub mod user;
pub mod post;
pub mod comment;
pub mod vote;
pub mod bookmark;
pub mod follow;
pub mod tag;
pub mod notification;
pub mod admin;
pub mod ws;

use axum::Router;
use std::sync::Arc;

use crate::models::AppState;
use self::auth::auth_routes;
use self::user::user_routes;
use self::post::post_routes;
use self::comment::comment_routes;
use self::vote::vote_routes;
use self::bookmark::bookmark_routes;
use self::follow::follow_routes;
use self::tag::tag_routes;
use self::notification::notification_routes;
use self::admin::admin_routes;
use self::ws::ws_routes;

pub fn app_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/posts", post_routes())
        .nest("/comments", comment_routes())
        .nest("/votes", vote_routes())
        .nest("/bookmarks", bookmark_routes())
        .nest("/follow", follow_routes())
        .nest("/tags", tag_routes())
        .nest("/notifications", notification_routes())
        .nest("/admin", admin_routes())
        .nest("/ws", ws_routes())
}
