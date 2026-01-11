mod config;
mod db;
mod models;
mod routes;
mod templates;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let pool = db::connect(&config).await?;

    // Session layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::hours(24),
        ));

    let app = Router::new()
        .merge(routes::public::router())
        .merge(routes::admin::router())
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(ServiceBuilder::new().layer(session_layer))
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", config.port)
        .parse()
        .unwrap();

    tracing::info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

