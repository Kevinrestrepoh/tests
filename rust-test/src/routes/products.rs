use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::models::products::{CreateProduct, Product};

pub fn product_routes() -> Router<PgPool> {
    Router::new()
        .route("/products", get(get_products))
        .route("/products", post(add_product))
}

async fn get_products(State(pool): State<PgPool>) -> Json<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>("SELECT id, name, price FROM products")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(products)
}

async fn add_product(
    State(pool): State<PgPool>,
    Json(product): Json<CreateProduct>,
) -> Json<Product> {
    let inserted = sqlx::query_as::<_, Product>(
        "INSERT INTO products(name, price) VALUES ($1, $2) RETURNING id, name, price",
    )
    .bind(product.name)
    .bind(product.price)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(inserted)
}
