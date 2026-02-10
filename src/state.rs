use crate::models::{LightState, PowerAction, LightCommand};
use log::{info, warn};
use tokio::sync::{mpsc, watch};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Message {
    User(LightCommand),
    MqttUpdate(String, LightState),
}

pub struct LightActor {
    states: HashMap<String, LightState>,
    cmd_rx: mpsc::Receiver<Message>,
    watch_tx: watch::Sender<HashMap<String, LightState>>,
    mqtt_client: rumqttc::AsyncClient,
}

impl LightActor {
    pub fn new(
        initial_state: HashMap<String, LightState>,
        cmd_rx: mpsc::Receiver<Message>,
        watch_tx: watch::Sender<HashMap<String, LightState>>,
        mqtt_client: rumqttc::AsyncClient,
    ) -> Self {
        Self { states: initial_state, cmd_rx, watch_tx, mqtt_client }
    }

    pub async fn run(mut self) {
        let _ = self.mqtt_client.subscribe("stat/+/power", rumqttc::QoS::AtLeastOnce).await;

        while let Some(msg) = self.cmd_rx.recv().await {
            match msg {
                Message::User(command) => {
                    info!("Light Actor: User command received");
                    info!("--- {command:?} ---");
                    if let Some(current) = self.states.get_mut(&command.id) {
                        *current = LightState::Pending;
                        let topic = format!("cmd/{}/power", command.id);
                        let payload = match command.cmd {
                            PowerAction::On => "ON",
                            PowerAction::Off => "OFF",
                        };
                        let _ = self.mqtt_client.publish(topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                    } else {
                        warn!("Light Actor: Device ID '{}' not found", command.id);
                    }
                }
                Message::MqttUpdate(id, new_state) => {
                    if self.states.insert(id.clone(), new_state).is_none() {
                        info!("Light Actor: New device added '{id}'");
                    }
                }
            }
            let _ = self.watch_tx.send(self.states.clone());
        }
    }
}