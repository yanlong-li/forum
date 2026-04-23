use sqlx::SqlitePool;

use crate::config::Config;

pub mod user;
pub mod post;
pub mod comment;
pub mod vote;
pub mod follow;
pub mod notification;
pub mod tag;
pub mod report;

pub mod response;

pub use user::*;
pub use post::*;
pub use comment::*;
pub use vote::*;
pub use follow::*;
pub use notification::*;
pub use tag::*;
pub use report::*;
pub use response::*;

pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}
