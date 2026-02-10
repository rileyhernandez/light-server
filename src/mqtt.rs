use crate::models::LightState;
use rumqttc::{Event, Packet};
use tokio::sync::mpsc;
use crate::state::Message;

impl TryFrom<&[u8]> for LightState {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(value).map(str::trim) {
            Ok("ON") => Ok(LightState::On),
            Ok("OFF") => Ok(LightState::Off),
            _ => Err(()),
        }
    }
}

pub async fn run_mqtt_loop(mut eventloop: rumqttc::EventLoop, tx: mpsc::Sender<Message>) {
    while let Ok(notification) = eventloop.poll().await {
        if let Event::Incoming(Packet::Publish(p)) = notification {
            let parts: Vec<&str> = p.topic.split('/').collect();
            // Expecting: stat/node-id/power
            if parts.len() == 3 && parts[0] == "stat" {
                let id = parts[1].to_string();
                if let Ok(new_state) = LightState::try_from(&p.payload[..]) {
                    let _ = tx.send(Message::MqttUpdate(id, new_state)).await;
                }
            }
        }
    }
}