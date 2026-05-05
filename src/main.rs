use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::any,
};

#[tokio::main]
async fn main() {
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap(),
        Router::new()
            .route("/ws", any(ws_handler))
            .into_make_service(),
    )
    .await
    .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    socket.send(Message::Text("hola".into())).await.unwrap();
    loop {
        let message = socket.recv().await.unwrap().unwrap();
        dbg!(&message);
        socket.send(message).await.unwrap();
    }
}
