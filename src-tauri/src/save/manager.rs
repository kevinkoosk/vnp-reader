use super::schema::*;
use crate::machine::RuntimeMachine;
use crate::prng::VnpPrng1;
use crate::state::StateRegistry;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("JSON serialization or deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Incompatible work ID: expected {expected}, found {found}")]
    WorkMismatch { expected: String, found: String },

    #[error("Incompatible save compatibility ID: expected {expected}, found {found}")]
    CompatibilityFamilyMismatch { expected: String, found: String },

    #[error("Unsupported save format version: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid PRNG state in save document (cannot be 0)[cite: 6]")]
    InvalidPrngState,

    #[error("Missing target scene during restoration: {0}")]
    MissingScene(String),

    #[error("Missing target event during restoration: {0}")]
    MissingEvent(String),
}

pub struct SaveManager;

impl SaveManager {
    /// Serialize the current runtime machine state into a portable JSON string[cite: 6]
    pub fn create_save(
        machine: &RuntimeMachine,
        work: WorkIdentity,
        locale: String,
        created_timestamp_rfc3339: String,
        snapshot: PresentationSnapshot,
        pending_event: Option<String>,
        pending_kind: Option<String>,
    ) -> Result<String, SaveError> {
        let current_event_id = machine.get_current_event_id();

        let pending = match (pending_kind, pending_event) {
            (Some(kind), Some(event)) => Some(PendingInteraction { kind, event }),
            _ => None,
        };

        let save_doc = PortableSaveDocument {
            schema: SAVE_SCHEMA_URI.to_string(),
            save_format: SAVE_FORMAT_VERSION.to_string(),
            work,
            created: created_timestamp_rfc3339,
            locale,
            position: SavePosition {
                scene: machine.current_scene_id.clone(),
                event: current_event_id,
                phase: "before".to_string(),
            },
            run_state: machine.run_state.values.clone(),
            call_stack: machine.call_stack.clone(),
            random: VnpPrng1Save {
                algorithm: "vnp-prng1".to_string(),
                state: machine.prng.state,
            },
            presentation: snapshot,
            pending,
            history_tail: None,
        };

        Ok(serde_json::to_string_pretty(&save_doc)?)
    }

    /// Restore machine state and presentation snapshot from a portable JSON save[cite: 6]
    pub fn restore_save(
        save_json: &str,
        machine: &mut RuntimeMachine,
        expected_work_id: &str,
        expected_compat_id: &str,
    ) -> Result<PresentationSnapshot, SaveError> {
        let doc: PortableSaveDocument = serde_json::from_str(save_json)?;

        // 1. Validate work and compatibility IDs
        if doc.work.id != expected_work_id {
            return Err(SaveError::WorkMismatch {
                expected: expected_work_id.to_string(),
                found: doc.work.id,
            });
        }

        if doc.work.compatibility_id != expected_compat_id {
            return Err(SaveError::CompatibilityFamilyMismatch {
                expected: expected_compat_id.to_string(),
                found: doc.work.compatibility_id,
            });
        }

        // 2. Validate PRNG seed[cite: 6]
        if doc.random.state == 0 {
            return Err(SaveError::InvalidPrngState);
        }

        // 3. Confirm scene and event exist in current publication[cite: 6]
        let scene = machine
            .scenes
            .get(&doc.position.scene)
            .ok_or_else(|| SaveError::MissingScene(doc.position.scene.clone()))?;

        let event_idx = scene
            .events
            .iter()
            .position(|e| machine.extract_event_id(e) == doc.position.event)
            .ok_or_else(|| SaveError::MissingEvent(doc.position.event.clone()))?;

        // 4. Atomically commit restored machine state[cite: 6]
        machine.current_scene_id = doc.position.scene;
        machine.current_event_index = event_idx;
        machine.run_state = StateRegistry { values: doc.run_state };
        machine.call_stack = doc.call_stack;
        machine.prng = VnpPrng1 { state: doc.random.state };

        // 5. Return the presentation snapshot for the UI layer to render without replaying events[cite: 6]
        Ok(doc.presentation)
    }
}
