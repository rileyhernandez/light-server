use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LightState {
    On,
    Off,
    Pending,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PowerAction {
    On,
    Off,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LightCommand {
    pub id: String,
    pub cmd: PowerAction,
}