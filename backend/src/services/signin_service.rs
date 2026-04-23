use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use uuid::Uuid;

pub struct SigninService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SigninService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn signin(&self, user_id: &str) -> Result<SigninResponse> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let existing: Option<(String,)> = sqlx::query_as(
            "SELECT signup_date FROM user_signups WHERE user_id = ? AND signup_date = ?"
        )
        .bind(user_id)
        .bind(&today)
        .fetch_optional(self.pool)
        .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Already signed in today".to_string()));
        }

        let consecutive_days = self.get_consecutive_days(user_id, &today).await?;
        let new_consecutive = consecutive_days + 1;

        let signin_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO user_signups (id, user_id, signup_date, consecutive_days, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&signin_id)
        .bind(user_id)
        .bind(&today)
        .bind(new_consecutive)
        .bind(&now)
        .execute(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to sign in".to_string()))?;

        let points_earned = self.calculate_signin_points(new_consecutive);

        sqlx::query("UPDATE users SET points = points + ? WHERE id = ?")
            .bind(points_earned)
            .bind(user_id)
            .execute(self.pool)
            .await
            .map_err(|_| AppError::InternalError("Failed to update points".to_string()))?;

        self.update_level(user_id).await?;

        Ok(SigninResponse {
            consecutive_days: new_consecutive,
            points_earned,
            total_points: self.get_user_points(user_id).await?,
        })
    }

    pub async fn get_signin_status(&self, user_id: &str) -> Result<SigninStatus> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let signed_in_today: Option<(String,)> = sqlx::query_as(
            "SELECT signup_date FROM user_signups WHERE user_id = ? AND signup_date = ?"
        )
        .bind(user_id)
        .bind(&today)
        .fetch_optional(self.pool)
        .await?;

        let consecutive_days = self.get_consecutive_days(user_id, &today).await?;

        Ok(SigninStatus {
            signed_in_today: signed_in_today.is_some(),
            consecutive_days,
        })
    }

    async fn get_consecutive_days(&self, user_id: &str, today: &str) -> Result<i64> {
        let last_signin: Option<(String, i64)> = sqlx::query_as(
            "SELECT signup_date, consecutive_days FROM user_signups WHERE user_id = ? ORDER BY signup_date DESC LIMIT 1"
        )
        .bind(user_id)
        .fetch_optional(self.pool)
        .await?;

        if let Some((date, days)) = last_signin {
            let yesterday = chrono::Utc::now()
                .checked_sub_signed(chrono::Duration::days(1))
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();

            if date == today {
                return Ok(days);
            } else if date == yesterday {
                return Ok(days);
            } else {
                return Ok(0);
            }
        }

        Ok(0)
    }

    fn calculate_signin_points(&self, consecutive_days: i64) -> i64 {
        let base = 5;
        let bonus = (consecutive_days / 7).min(5);
        base + bonus
    }

    async fn get_user_points(&self, user_id: &str) -> Result<i64> {
        let result: (i64,) = sqlx::query_as("SELECT points FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(self.pool)
            .await
            .map_err(|_| AppError::InternalError("Failed to get points".to_string()))?;
        Ok(result.0)
    }

    async fn update_level(&self, user_id: &str) -> Result<()> {
        let points: (i64,) = sqlx::query_as("SELECT points FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(self.pool)
            .await
            .map_err(|_| AppError::InternalError("Failed to get points".to_string()))?;

        let level = calculate_level(points.0);

        sqlx::query("UPDATE users SET level = ? WHERE id = ?")
            .bind(level)
            .bind(user_id)
            .execute(self.pool)
            .await
            .map_err(|_| AppError::InternalError("Failed to update level".to_string()))?;

        Ok(())
    }
}

pub fn calculate_level(points: i64) -> i64 {
    if points < 100 { 1 }
    else if points < 500 { 2 }
    else if points < 1000 { 3 }
    else if points < 2500 { 4 }
    else if points < 5000 { 5 }
    else if points < 10000 { 6 }
    else if points < 25000 { 7 }
    else if points < 50000 { 8 }
    else { 9 }
}

pub fn calculate_points_for_action(action: &str) -> i64 {
    match action {
        "create_post" => 10,
        "create_comment" => 5,
        "receive_like" => 2,
        "receive_follow" => 3,
        "best_answer" => 15,
        _ => 0,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SigninResponse {
    pub consecutive_days: i64,
    pub points_earned: i64,
    pub total_points: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SigninStatus {
    pub signed_in_today: bool,
    pub consecutive_days: i64,
}
