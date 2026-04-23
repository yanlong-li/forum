use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::comment::{Comment, CreateCommentRequest, UpdateCommentRequest, CommentListResponse};
use crate::models::response::MessageResponse;
use crate::repositories::{CommentRepository, PostRepository, NotificationRepository};
use crate::services::utils::extract_mentions;
use crate::services::sensitive_words::SensitiveWords;
use uuid::Uuid;

pub struct CommentService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> CommentService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_comment(&self, post_id: &str, author_id: &str, req: CreateCommentRequest) -> Result<Comment> {
        if req.content.is_empty() {
            return Err(AppError::ValidationError("Comment content is required".to_string()));
        }

        let sensitive = SensitiveWords::new();
        if sensitive.contains_sensitive(&req.content) {
            return Err(AppError::ValidationError("Comment contains sensitive words".to_string()));
        }

        let comment_repo = CommentRepository::new(self.pool);
        let post_repo = PostRepository::new(self.pool);

        let post = post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if let Some(parent_id) = &req.parent_id {
            let parent = comment_repo.find_by_id(parent_id).await?
                .ok_or_else(|| AppError::NotFound("Parent comment not found".to_string()))?;
            if parent.post_id != post_id {
                return Err(AppError::ValidationError("Parent comment does not belong to this post".to_string()));
            }
        }

        let comment_id = Uuid::new_v4().to_string();
        let comment = comment_repo.create(&comment_id, post_id, author_id, req.parent_id.as_deref(), &req.content).await?;

        if post.author_id != author_id {
            let notification_repo = NotificationRepository::new(self.pool);
            let notif_id = Uuid::new_v4().to_string();
            let notif_type = if req.parent_id.is_some() { "reply" } else { "comment" };
            notification_repo.create(&notif_id, &post.author_id, notif_type, author_id, Some(post_id), Some(&comment_id)).await?;
        }

        let mentioned_usernames = extract_mentions(&req.content);
        if !mentioned_usernames.is_empty() {
            let user_repo = crate::repositories::UserRepository::new(self.pool);
            let notification_repo = NotificationRepository::new(self.pool);

            for username in mentioned_usernames {
                if let Some(mentioned_user) = user_repo.find_by_username(&username).await? {
                    if mentioned_user.id != author_id && mentioned_user.id != post.author_id {
                        let notif_id = Uuid::new_v4().to_string();
                        notification_repo.create(&notif_id, &mentioned_user.id, "mention", author_id, Some(post_id), Some(&comment_id)).await?;
                    }
                }
            }
        }

        Ok(comment)
    }

    pub async fn update_comment(&self, id: &str, author_id: &str, req: UpdateCommentRequest) -> Result<Comment> {
        if req.content.is_empty() {
            return Err(AppError::ValidationError("Comment content is required".to_string()));
        }

        let comment_repo = CommentRepository::new(self.pool);

        let comment = comment_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

        if comment.author_id != author_id {
            return Err(AppError::Forbidden("Not authorized to update this comment".to_string()));
        }

        comment_repo.update(id, &req.content).await
    }

    pub async fn delete_comment(&self, id: &str, author_id: &str) -> Result<()> {
        let comment_repo = CommentRepository::new(self.pool);

        let comment = comment_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

        if comment.author_id != author_id {
            return Err(AppError::Forbidden("Not authorized to delete this comment".to_string()));
        }

        comment_repo.delete(id).await
    }

    pub async fn list_comments(&self, post_id: &str, page: i64, limit: i64) -> Result<CommentListResponse> {
        let comment_repo = CommentRepository::new(self.pool);
        let (comments, total) = comment_repo.list_by_post(post_id, page, limit).await?;

        let mut result = Vec::new();
        for mut comment in comments {
            let replies = comment_repo.list_replies(&comment.id).await?;
            comment.reply_count = replies.len() as i64;
            result.push(comment);
        }

        Ok(CommentListResponse {
            comments: result,
            total,
            page,
            limit,
        })
    }

    pub async fn list_user_comments(&self, user_id: &str, page: i64, limit: i64) -> Result<CommentListResponse> {
        let comment_repo = CommentRepository::new(self.pool);
        let (comments, total) = comment_repo.list_by_author(user_id, page, limit).await?;

        Ok(CommentListResponse {
            comments,
            total,
            page,
            limit,
        })
    }

    pub async fn accept_comment(&self, comment_id: &str, user_id: &str) -> Result<MessageResponse> {
        let comment_repo = CommentRepository::new(self.pool);
        let post_repo = PostRepository::new(self.pool);

        let comment = comment_repo.find_by_id(comment_id).await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

        let post = post_repo.find_by_id(&comment.post_id).await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if post.author_id != user_id {
            return Err(AppError::Forbidden("Only post author can accept comments".to_string()));
        }

        comment_repo.unaccept_all_for_post(&comment.post_id).await?;
        comment_repo.accept(comment_id).await?;

        Ok(MessageResponse {
            message: "Comment accepted".to_string()
        })
    }
}
