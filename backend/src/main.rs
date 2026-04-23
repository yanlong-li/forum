use std::sync::Arc;
use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

pub mod config;
pub mod error;
pub mod extractors;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod ws;
pub mod oauth;

use config::Config;
use routes::app_routes;
use middleware::cors::cors_layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("community_forum=info".parse()?)
        )
        .init();

    let config = Config::from_env()?;
    let db = sqlx::SqlitePool::connect(&config.database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let app_state = Arc::new(models::AppState { db, config: config.clone() });

    let app = Router::new()
        .nest("/api/v1", app_routes())
        .layer(CompressionLayer::new())
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
