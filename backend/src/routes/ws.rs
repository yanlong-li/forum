use axum::{Router, routing::get, extract::{Query, WebSocketUpgrade, State}};
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};

use crate::models::AppState;
use crate::error::Result;

pub fn ws_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(ws_handler))
}

async fn ws_handler(
    State(_state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl axum::response::IntoResponse> {
    let _token = params.get("token").cloned();

    Ok(ws.on_upgrade(move |socket| async move {
        handle_socket(socket).await;
    }))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                tracing::debug!("WebSocket received: {}", text);
            }
            Ok(axum::extract::ws::Message::Binary(data)) => {
                tracing::debug!("WebSocket received binary: {} bytes", data.len());
            }
            Ok(axum::extract::ws::Message::Ping(data)) => {
                if sender.send(axum::extract::ws::Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Ok(axum::extract::ws::Message::Pong(_)) => {}
            Ok(axum::extract::ws::Message::Close(None)) | Ok(axum::extract::ws::Message::Close(Some(_))) => {
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
