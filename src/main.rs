use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use init_tracing_opentelemetry::tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing::{span, Level};

#[tokio::main]
async fn main() {
    let _guard = init_tracing_opentelemetry::TracingConfig::production()
        .init_subscriber()
        .unwrap();
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        Router::new()
            .route("/ws", any(ws_handler))
            .layer(OtelInResponseLayer::default())
            .layer(OtelAxumLayer::default())
            .into_make_service(),
    )
    .await
    .unwrap();
}

#[tracing::instrument]
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

#[tracing::instrument]
async fn handle_socket(mut socket: WebSocket) {
    let span = span!(Level::INFO, "initial");
    {
        let _enter = span.enter();
        socket.send(Message::Text("hola".into())).await.unwrap();
    }
    loop {
        let span = span!(Level::INFO, "ping");
        let _enter = span.enter();
        let message = socket.recv().await.unwrap().unwrap();
        span.set_attribute("message", message.clone().into_text().unwrap().to_string());
        {
            let span = span!(Level::INFO, "pong");
            let _enter = span.enter();
            socket.send(message).await.unwrap();
        }
    }
}
