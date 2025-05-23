use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;

mod router;
mod policy;

#[tokio::main]
async fn main() {
    // Inizializza il logger/tracing
    tracing_subscriber::fmt::init();

    // Costruisci le route
    let app = Router::new()
        .route("/v1/chat/completions", post(router::proxy))
        .layer(TraceLayer::new_for_http());

    // Prepara il listener TCP (0.0.0.0:8080)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("impossibile bindare la porta 8080");

    tracing::info!("🔐  LLM Gateway listening on {addr}");

    // Nuova API axum 0.7: serve(listener, app)
    axum::serve(listener, app)
        .await
        .expect("server failed");
}