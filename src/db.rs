use sqlx::{Pool, Postgres};

use crate::data::{User, UserCredentials};

pub async fn add_user(
    user_credentials: UserCredentials,
    user: User,
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO user_credentials (username, auth_token) VALUES ($1, $2)",
        user_credentials.username,
        user_credentials.auth_token
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "INSERT INTO users (username, public_key) VALUES ($1, $2)",
        user.username,
        user.public_key
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn user_exists(username: &str, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT 1 as exists FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
