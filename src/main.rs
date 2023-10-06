use axum::{Router, Server};
use dotenvy::dotenv;
use persistence::database_controller::DatabaseController;
use sqlx::postgres::PgPoolOptions;
use web::routes_q_and_a::routes;

// Modules
mod error;
mod handlers;
pub mod model;
mod persistence;
mod web;

// Self/Crate
pub use self::error::{Error, Result};

// Standard

// Dependencies

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    let dc = DatabaseController::new(db_pool)?;

    let app: Router = Router::new().merge(routes(dc));

    Server::bind(&"0.0.0.0:1337".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    return Ok(());
}
