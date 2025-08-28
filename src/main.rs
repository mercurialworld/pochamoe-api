use axum::{
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::{health::handler as health_handler, version::handler as version_handler};

mod routes;

#[tokio::main]
async fn main() {
     tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/v1/version/{mod_name}/{bs_version}", get(version_handler))
        .route("/health", get(health_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9669")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

