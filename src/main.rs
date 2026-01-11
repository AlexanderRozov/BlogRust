use blog::*;

use axum::{Router, http::StatusCode, error_handling::HandleErrorLayer};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer};
use tracing_subscriber;
use std::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = connect(&config).await?;

    // Session layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::hours(24),
        ));

    async fn handle_session_error(err: Box<dyn Error + Send + Sync>) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Session error: {}", err),
        )
    }

    let app = Router::new()
        .merge(blog::routes::public::router())
        .merge(blog::routes::admin::router())
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_session_error))
                .layer(session_layer)
        )
        .with_state(pool);

    let addr: std::net::SocketAddr = format!("0.0.0.0:{}", config.port)
        .parse()
        .unwrap();

    tracing::info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

