use serde::{Deserialize, Serialize};

use super::app::AppId;
use crate::ai::CellState;
use crate::map::Position;
use ndarray::Array2;

pub type MsgId = u32;

/// Defines message type
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgContent {
    Private(AppId, String),
    Public(String),
    MapUpdate(Array2<CellState>),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Msg {
    pub id: MsgId,
    pub sender_id: AppId,
    pub pos: Position,
    pub content: MsgContent,
}

impl Msg {
    pub fn new(sender_id: AppId, pos: Position, content: MsgContent) -> Self {
        Msg {
            id: rand::random(),
            sender_id,
            content,
            pos,
        }
    }
    pub fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
    pub fn from_str(json: &str) -> serde_json::Result<Msg> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_serde() {
        let msg = Msg {
            id: rand::random(),
            sender_id: rand::random(),
            pos: Position::default(),
            content: MsgContent::Private(rand::random(), "I like trains !".to_string()),
        };

        let serialized = msg.serialize().expect("failed to serialize");
        println!("serialized = {}", serialized);

        // Convert the JSON string back to a Msg.
        let deserialized = Msg::from_str(&serialized).expect("failed to deserialize");

        println!("deserialized = {:?}", deserialized);

        assert_eq!(msg, deserialized);
    }
}
