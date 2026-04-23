use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::post::{Post, PostWithAuthor, CreatePostRequest, UpdatePostRequest, PostListResponse};
use crate::models::user::UserPublic;
use crate::repositories::{PostRepository, TagRepository, UserRepository};
use uuid::Uuid;

pub struct PostService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> PostService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_post(&self, author_id: &str, req: CreatePostRequest) -> Result<Post> {
        if req.title.is_empty() {
            return Err(AppError::ValidationError("Title is required".to_string()));
        }
        if req.content.len() < 10 {
            return Err(AppError::ValidationError("Content must be at least 10 characters".to_string()));
        }
        if req.tags.is_empty() {
            return Err(AppError::ValidationError("At least one tag is required".to_string()));
        }

        let post_repo = PostRepository::new(self.pool);
        let tag_repo = TagRepository::new(self.pool);

        let post_id = Uuid::new_v4().to_string();
        let post = post_repo.create(&post_id, author_id, &req.title, &req.content).await?;

        for tag_name in &req.tags {
            let tag_id = Uuid::new_v4().to_string();
            let tag = tag_repo.get_or_create(&tag_id, tag_name).await?;
            post_repo.add_tag(&post_id, &tag.id).await?;
            tag_repo.increment_post_count(&tag.id).await?;
        }

        Ok(post)
    }

    pub async fn get_post(&self, id: &str, _current_user_id: Option<&str>) -> Result<PostWithAuthor> {
        let post_repo = PostRepository::new(self.pool);
        let user_repo = UserRepository::new(self.pool);

        let post = post_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        let author = user_repo.find_by_id(&post.author_id).await?
            .ok_or_else(|| AppError::NotFound("Author not found".to_string()))?;

        let (like_count, dislike_count) = self.get_post_vote_counts(id).await?;

        Ok(PostWithAuthor {
            id: post.id,
            author_id: post.author_id,
            author: UserPublic {
                id: author.id,
                username: author.username,
                avatar_url: author.avatar_url,
                bio: author.bio,
                is_admin: author.is_admin,
                created_at: author.created_at,
            },
            title: post.title,
            content: post.content,
            tags: vec![],
            like_count,
            dislike_count,
            comment_count: 0,
            is_bookmarked: false,
            is_liked: false,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    pub async fn update_post(&self, id: &str, author_id: &str, req: UpdatePostRequest) -> Result<Post> {
        let post_repo = PostRepository::new(self.pool);

        let post = post_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if post.author_id != author_id {
            return Err(AppError::Forbidden("Not authorized to update this post".to_string()));
        }

        post_repo.update(id, req.title.as_deref(), req.content.as_deref()).await
    }

    pub async fn delete_post(&self, id: &str, author_id: &str) -> Result<()> {
        let post_repo = PostRepository::new(self.pool);

        let post = post_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if post.author_id != author_id {
            return Err(AppError::Forbidden("Not authorized to delete this post".to_string()));
        }

        post_repo.delete(id).await
    }

    pub async fn list_posts(&self, page: i64, limit: i64, tag: Option<&str>, sort: Option<&str>) -> Result<PostListResponse> {
        let post_repo = PostRepository::new(self.pool);
        let (posts, total) = post_repo.list(page, limit, tag, sort).await?;

        Ok(PostListResponse {
            posts,
            total,
            page,
            limit,
        })
    }

    pub async fn list_user_posts(&self, author_id: &str, page: i64, limit: i64) -> Result<PostListResponse> {
        let post_repo = PostRepository::new(self.pool);
        let (posts, total) = post_repo.list_by_author(author_id, page, limit).await?;

        Ok(PostListResponse {
            posts,
            total,
            page,
            limit,
        })
    }

    async fn get_post_vote_counts(&self, post_id: &str) -> Result<(i64, i64)> {
        use crate::repositories::VoteRepository;
        let vote_repo = VoteRepository::new(self.pool);
        vote_repo.get_post_counts(post_id).await
    }
}
