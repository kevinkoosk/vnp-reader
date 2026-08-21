use crate::schema::EventTarget;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SAVE_FORMAT_VERSION: &str = "1.0.0";
pub const SAVE_SCHEMA_URI: &str = "https://vnp.example/spec/1.0/save.schema.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkIdentity {
    pub id: String,
    pub version: String,
    pub compatibility_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePosition {
    pub scene: String,
    pub event: String,
    pub phase: String, // Normatively "before"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedObject {
    pub id: String,
    pub expression: Option<String>,
    pub position: Option<String>,
    pub layer: String, // "rear" | "stage" | "front" | "interface"
    pub scale: f64,
    pub opacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: f64,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, scale: 1.0, rotation: 0.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainedAudioState {
    pub resource: String,
    pub volume: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PresentationSnapshot {
    pub background: Option<String>,
    pub objects: Vec<StagedObject>,
    pub camera: CameraState,
    pub theme: String,
    pub nvl_page: Vec<String>,
    pub music: Option<SustainedAudioState>,
    pub ambient: Option<SustainedAudioState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortableSaveDocument {
    pub schema: String,
    pub save_format: String,
    pub work: WorkIdentity,
    pub created: String, // ISO 8601 / RFC 3339 UTC timestamp
    pub locale: String,
    pub position: SavePosition,
    pub run_state: HashMap<String, crate::state::ScalarValue>,
    pub call_stack: Vec<EventTarget>,
    pub random: VnpPrng1Save,
    pub presentation: PresentationSnapshot,
    pub pending: Option<PendingInteraction>,
    pub history_tail: Option<Vec<HistoryTailEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VnpPrng1Save {
    pub algorithm: String, // Normatively "vnp-prng1"
    pub state: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingInteraction {
    pub kind: String, // "advance" | "choice" | "input"[cite: 6]
    pub event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryTailEntry {
    pub scene: String,
    pub event: String,
}
