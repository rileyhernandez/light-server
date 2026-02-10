mod models;
mod state;
mod mqtt;
mod handlers;

use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;
use std::{collections::HashMap, time::Duration};
use tokio::sync::{mpsc, watch};
use crate::models::LightState;
use crate::state::{LightActor, Message};
use log::{info};

#[derive(Clone)]
pub struct AppState {
    pub actor_tx: mpsc::Sender<Message>,
    pub state_rx: watch::Receiver<HashMap<String, LightState>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let mut initial_state = HashMap::new();
    initial_state.insert("node-0".into(), LightState::Off);
    info!("Initial state: {initial_state:?}");
    
    let mut mqtt_options = rumqttc::MqttOptions::new("axum-backend", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, eventloop) = rumqttc::AsyncClient::new(mqtt_options, 10);

    let (actor_tx, actor_rx) = mpsc::channel(10);
    let (state_tx, state_rx) = watch::channel(initial_state.clone());

    let actor = LightActor::new(initial_state, actor_rx, state_tx, mqtt_client);
    
    info!("Spawning Light Actor...");
    tokio::spawn(actor.run());
    info!("Success!");
    
    info!("Spawning MQTT Actor...");
    tokio::spawn(mqtt::run_mqtt_loop(eventloop, actor_tx.clone()));
    info!("Success!");

    let app_state = AppState { actor_tx, state_rx };

    let app = Router::new()
        .route("/update", post(handlers::update_state))
        .route("/state", get(handlers::get_state))
        .route("/ws", get(handlers::ws_handler))
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true))
        .with_state(app_state);
    
    info!("Opening TCP Socket...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Ready to receive commands!");
    axum::serve(listener, app).await.unwrap();
}