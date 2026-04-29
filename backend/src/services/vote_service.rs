use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::vote::{VoteRequest, VoteResponse};
use crate::repositories::VoteRepository;
use uuid::Uuid;

pub struct VoteService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> VoteService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn vote(&self, user_id: &str, req: VoteRequest) -> Result<VoteResponse> {
        if req.post_id.is_none() && req.comment_id.is_none() {
            return Err(AppError::ValidationError("Must specify post_id or comment_id".to_string()));
        }

        if req.value != 1 && req.value != -1 {
            return Err(AppError::ValidationError("Value must be 1 or -1".to_string()));
        }

        let vote_repo = VoteRepository::new(self.pool);

        if let Some(post_id) = &req.post_id {
            if let Some(existing) = vote_repo.find_user_post_vote(user_id, post_id).await? {
                if existing.value == req.value {
                    vote_repo.delete(&existing.id).await?;
                    let (like_count, dislike_count) = vote_repo.get_post_counts(post_id).await?;
                    return Ok(VoteResponse {
                        post_id: req.post_id,
                        comment_id: req.comment_id,
                        like_count,
                        dislike_count,
                    });
                } else {
                    vote_repo.update(&existing.id, req.value).await?;
                    let (like_count, dislike_count) = vote_repo.get_post_counts(post_id).await?;
                    return Ok(VoteResponse {
                        post_id: req.post_id,
                        comment_id: req.comment_id,
                        like_count,
                        dislike_count,
                    });
                }
            }

            let vote_id = Uuid::new_v4().to_string();
            vote_repo.create(&vote_id, user_id, Some(post_id), None, req.value).await?;
            let (like_count, dislike_count) = vote_repo.get_post_counts(post_id).await?;

            Ok(VoteResponse {
                post_id: req.post_id,
                comment_id: req.comment_id,
                like_count,
                dislike_count,
            })
        } else if let Some(comment_id) = &req.comment_id {
            if let Some(existing) = vote_repo.find_user_comment_vote(user_id, comment_id).await? {
                if existing.value == req.value {
                    vote_repo.delete(&existing.id).await?;
                    let (like_count, dislike_count) = vote_repo.get_comment_counts(comment_id).await?;
                    return Ok(VoteResponse {
                        post_id: req.post_id,
                        comment_id: req.comment_id,
                        like_count,
                        dislike_count,
                    });
                } else {
                    vote_repo.update(&existing.id, req.value).await?;
                    let (like_count, dislike_count) = vote_repo.get_comment_counts(comment_id).await?;
                    return Ok(VoteResponse {
                        post_id: req.post_id,
                        comment_id: req.comment_id,
                        like_count,
                        dislike_count,
                    });
                }
            }

            let vote_id = Uuid::new_v4().to_string();
            vote_repo.create(&vote_id, user_id, None, Some(comment_id), req.value).await?;
            let (like_count, dislike_count) = vote_repo.get_comment_counts(comment_id).await?;

            Ok(VoteResponse {
                post_id: req.post_id,
                comment_id: req.comment_id,
                like_count,
                dislike_count,
            })
        } else {
            Err(AppError::ValidationError("Must specify post_id or comment_id".to_string()))
        }
    }

    pub async fn unvote(&self, user_id: &str, post_id: &str) -> Result<VoteResponse> {
        let vote_repo = VoteRepository::new(self.pool);

        vote_repo.delete_by_post(user_id, post_id).await?;
        let (like_count, dislike_count) = vote_repo.get_post_counts(post_id).await?;

        Ok(VoteResponse {
            post_id: Some(post_id.to_string()),
            comment_id: None,
            like_count,
            dislike_count,
        })
    }
}
