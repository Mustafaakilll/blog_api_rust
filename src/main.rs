use std::net::SocketAddr;

use axum::Router;
use hyper::Server;

use config::Config;
use posts::router::PostRouter;
use user::router::UserRouter;

mod config;
mod posts;
mod state;
mod user;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::new();
    let config = &config;

    let db = sqlx::PgPool::connect(&config.database_url).await?;

    let state = state::AppState {
        db,
        config: config.clone(),
    };

    let app = Router::new()
        .nest("/user", UserRouter::new_router(state.clone()))
        .nest("/posts", PostRouter::new_router(state.clone()));

    let port = config.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    return Ok(());
}
