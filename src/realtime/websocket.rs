//! WebSocket server implementation for real-time updates.

use super::{RealtimeState, Subscription, UpdateMessage};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;
use tracing::{info, warn, error};

/// Handles WebSocket upgrade requests
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<RealtimeState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handles an individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<RealtimeState>) {
    let client_id = Uuid::new_v4().to_string();
    let (mut sender, mut receiver) = socket.split();
    
    // Create a channel for sending updates to this client
    let (tx, mut rx) = mpsc::channel::<UpdateMessage>(100);
    
    // Store the connection
    state.connections.insert(client_id.clone(), tx);
    
    info!("New WebSocket connection: {}", client_id);
    
    // Spawn task to handle outgoing messages
    let client_id_clone = client_id.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages
    while let Some(msg) = receiver.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                handle_client_message(&client_id, &text, &state).await;
            }
            Ok(Message::Close(_)) => {
                info!("Client {} disconnected", client_id);
                break;
            }
            Err(e) => {
                error!("WebSocket error for client {}: {}", client_id, e);
                break;
            }
            _ => {}
        }
    }
    
    // Clean up
    state.connections.remove(&client_id);
    state.subscriptions.remove(&client_id);
    send_task.abort();
    
    info!("Client {} cleanup complete", client_id);
}

/// Handles messages received from clients
async fn handle_client_message(
    client_id: &str,
    message: &str,
    state: &Arc<RealtimeState>,
) {
    #[derive(Debug, serde::Deserialize)]
    #[serde(tag = "type")]
    enum ClientMessage {
        Subscribe { subscription: Subscription },
        Unsubscribe,
        Ping,
    }
    
    match serde_json::from_str::<ClientMessage>(message) {
        Ok(ClientMessage::Subscribe { subscription }) => {
            info!("Client {} subscribed to: {:?}", client_id, subscription);
            state.subscriptions.insert(client_id.to_string(), subscription);
            
            // Send confirmation
            if let Some(sender) = state.connections.get(client_id) {
                let _ = sender.send(UpdateMessage::SystemUpdate {
                    message: "Subscription successful".to_string(),
                    severity: super::Severity::Info,
                }).await;
            }
        }
        Ok(ClientMessage::Unsubscribe) => {
            info!("Client {} unsubscribed", client_id);
            state.subscriptions.remove(client_id);
        }
        Ok(ClientMessage::Ping) => {
            // Send pong
            if let Some(sender) = state.connections.get(client_id) {
                let _ = sender.send(UpdateMessage::SystemUpdate {
                    message: "pong".to_string(),
                    severity: super::Severity::Info,
                }).await;
            }
        }
        Err(e) => {
            warn!("Invalid message from client {}: {}", client_id, e);
        }
    }
}

/// Creates the WebSocket router for Axum
pub fn create_websocket_router() -> axum::Router<Arc<RealtimeState>> {
    axum::Router::new()
        .route("/ws", axum::routing::get(websocket_handler))
}