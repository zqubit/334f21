use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Ping(String),
    Pong(String),
}
