use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::models::user::{CreateUser, User};

pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(add_user))
}

async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(users)
}

async fn add_user(State(pool): State<PgPool>, Json(user): Json<CreateUser>) -> Json<User> {
    let inserted = sqlx::query_as::<_, User>(
        "INSERT INTO users(name, password) VALUES ($1, $2) RETURNING id, name",
    )
    .bind(user.name)
    .bind(user.password)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(inserted)
}
