use std::net::SocketAddr;

use axum::Router;
use config::Config;
use hyper::Server;
use user::router::UserRouter;

mod config;
mod posts;
mod state;
mod user;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::new();

    let db = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let state = state::AppState { db, config };

    let app = Router::new().nest("/user", UserRouter::new_router(state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    return Ok(());
}
