use axum::{
    extract::{Json, State, ws::{Message as WsMsg, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;

use crate::AppState;
use crate::models::{LightCommand, LightState};
use crate::state::Message;
use log::{info};

/// POST /update
/// Receives a command from the frontend and forwards it to the ```LightActor```
pub async fn update_state(
    State(state): State<AppState>, 
    Json(payload): Json<LightCommand>
) {
    info!("Backend: Device '{}' requested transition to {:?}", payload.id, payload.cmd);
    // Send the command to the actor. We ignore the error if the actor is down.
    let _ = state.actor_tx.send(Message::User(payload)).await;
}

/// GET /state
/// Returns the current snapshot of all lights
pub async fn get_state(
    State(state): State<AppState>
) -> Json<HashMap<String, LightState>> {
    let current_state = state.state_rx.borrow().clone();
    Json(current_state)
}

/// GET /ws
/// Upgrades the connection to a WebSocket for real-time state updates
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Internal logic for managing the WebSocket lifecycle
// src/handlers.rs
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, _) = socket.split();
    let mut rx = state.state_rx.clone();

    // 1. Initial Send
    let initial_data = serde_json::to_string(&*rx.borrow()).unwrap();
    if sender.send(WsMsg::Text(initial_data.into())).await.is_err() {
        return;
    }

    // 2. The loop
    // Explicitly help the compiler see the return of changed()
    while let Ok(()) = rx.changed().await { 
        let data = serde_json::to_string(&*rx.borrow()).unwrap();
        if sender.send(WsMsg::Text(data.into())).await.is_err() {
            break;
        }
    }
}