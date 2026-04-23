use axum::{Router, Json, routing::get, extract::{Path, Query, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::repositories::TagRepository;
use crate::models::tag::{Tag, TagListResponse};
use crate::models::response::{PaginationQuery, PostListQuery};
use crate::error::Result;
use crate::services::PostService;

pub fn tag_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_tags))
        .route("/{name}", get(get_tag))
        .route("/{name}/posts", get(get_tag_posts))
}

async fn list_tags(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<TagListResponse>> {
    let tag_repo = TagRepository::new(&state.db);
    let (tags, total) = tag_repo.list(query.page(), query.limit(), None).await?;

    Ok(Json(TagListResponse {
        tags,
        total,
        page: query.page(),
        limit: query.limit(),
    }))
}

async fn get_tag(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Tag>> {
    let tag_repo = TagRepository::new(&state.db);
    let tag = tag_repo.find_by_name(&name).await?
        .ok_or_else(|| crate::error::AppError::NotFound("Tag not found".to_string()))?;

    Ok(Json(tag))
}

async fn get_tag_posts(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<PostListQuery>,
) -> Result<Json<crate::models::post::PostListResponse>> {
    let post_service = PostService::new(&state.db);
    post_service.list_posts(query.page.unwrap_or(1), query.limit.unwrap_or(20), Some(&name), query.sort.as_deref()).await.map(Json)
}
