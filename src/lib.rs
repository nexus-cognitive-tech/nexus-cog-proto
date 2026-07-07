//! Nexus Cog protocol contracts.
//!
//! Schema definitions live in `proto/`. This crate ships them alongside
//! Rust-side data structures that mirror the protobuf message shapes so
//! downstream crates can serialize without dragging in a protobuf runtime.

use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryItemProto {
    pub id: String,
    pub key: String,
    pub value: String,
    pub confidence: f64,
    pub access_count: u32,
    pub last_accessed: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomProto {
    pub id: String,
    pub name: String,
    pub room_type: String,
    pub importance: f64,
    pub items: Vec<MemoryItemProto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionProto {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PalaceSnapshotProto {
    pub id: String,
    pub name: String,
    pub rooms: Vec<RoomProto>,
    pub connections: Vec<ConnectionProto>,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_roundtrip() {
        let snap = PalaceSnapshotProto {
            id: "test".into(),
            name: "test".into(),
            rooms: vec![],
            connections: vec![],
            timestamp: 0,
        };
        let json = serde_json::to_string(&snap).unwrap();
        let back: PalaceSnapshotProto = serde_json::from_str(&json).unwrap();
        assert_eq!(snap, back);
    }
}
