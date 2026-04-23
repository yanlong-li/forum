use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

pub struct BadgeService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> BadgeService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_badges(&self) -> Result<Vec<Badge>> {
        let badges: Vec<Badge> = sqlx::query_as("SELECT * FROM badges")
            .fetch_all(self.pool)
            .await?;
        Ok(badges)
    }

    pub async fn get_user_badges(&self, user_id: &str) -> Result<Vec<UserBadgeWithDetails>> {
        let badges: Vec<UserBadgeWithDetails> = sqlx::query_as(
            r#"
            SELECT ub.id, ub.user_id, ub.earned_at,
                   b.id as badge_id, b.name, b.description, b.icon, b.criteria
            FROM user_badges ub
            JOIN badges b ON ub.badge_id = b.id
            WHERE ub.user_id = ?
            ORDER BY ub.earned_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        Ok(badges)
    }

    pub async fn check_and_award_badges(&self, user_id: &str) -> Result<Vec<Badge>> {
        let mut awarded_badges = Vec::new();

        let user_stats = self.get_user_stats(user_id).await?;

        let badges: Vec<Badge> = sqlx::query_as("SELECT * FROM badges")
            .fetch_all(self.pool)
            .await?;

        for badge in badges {
            let existing: Option<(String,)> = sqlx::query_as(
                "SELECT id FROM user_badges WHERE user_id = ? AND badge_id = ?"
            )
            .bind(user_id)
            .bind(&badge.id)
            .fetch_optional(self.pool)
            .await?;

            if existing.is_some() {
                continue;
            }

            if let Some(criteria) = &badge.criteria {
                if self.check_criteria(criteria, &user_stats).await? {
                    self.award_badge(user_id, &badge.id).await?;
                    awarded_badges.push(badge);
                }
            }
        }

        Ok(awarded_badges)
    }

    async fn get_user_stats(&self, user_id: &str) -> Result<UserStats> {
        let post_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM posts WHERE author_id = ? AND is_deleted = 0"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let comment_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM comments WHERE author_id = ? AND is_deleted = 0"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let follower_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM follows WHERE following_id = ?"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let likes_received: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE post_id IN (SELECT id FROM posts WHERE author_id = ?) AND value = 1"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let consecutive_signin: i64 = sqlx::query_scalar(
            "SELECT MAX(consecutive_days) FROM user_signups WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let created_at: String = sqlx::query_scalar(
            "SELECT created_at FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let member_days = chrono::Utc::now()
            .signed_duration_since(chrono::DateTime::parse_from_rfc3339(&created_at).unwrap())
            .num_days() as i64;

        Ok(UserStats {
            post_count,
            comment_count,
            follower_count,
            likes_received,
            consecutive_signin,
            member_days,
        })
    }

    async fn check_criteria(&self, criteria: &str, stats: &UserStats) -> Result<bool> {
        #[derive(Deserialize)]
        struct Criteria {
            #[serde(rename = "type")]
            criteria_type: String,
            value: i64,
        }

        let criteria: Criteria = serde_json::from_str(criteria)
            .map_err(|_| AppError::InternalError("Invalid criteria format".to_string()))?;

        match criteria.criteria_type.as_str() {
            "post_count" => Ok(stats.post_count >= criteria.value),
            "comment_count" => Ok(stats.comment_count >= criteria.value),
            "follower_count" => Ok(stats.follower_count >= criteria.value),
            "likes_received" => Ok(stats.likes_received >= criteria.value),
            "consecutive_signin" => Ok(stats.consecutive_signin >= criteria.value),
            "member_days" => Ok(stats.member_days >= criteria.value),
            _ => Ok(false),
        }
    }

    async fn award_badge(&self, user_id: &str, badge_id: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO user_badges (id, user_id, badge_id, earned_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(user_id)
        .bind(badge_id)
        .bind(&now)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub criteria: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBadge {
    pub id: String,
    pub user_id: String,
    pub badge_id: String,
    pub earned_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBadgeWithDetails {
    pub id: String,
    pub user_id: String,
    pub earned_at: String,
    pub badge_id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub criteria: Option<String>,
}

struct UserStats {
    post_count: i64,
    comment_count: i64,
    follower_count: i64,
    likes_received: i64,
    consecutive_signin: i64,
    member_days: i64,
}
