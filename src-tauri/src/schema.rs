use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDocument {
    pub id: String,
    pub mode: String,
    pub entry_event: String,
    pub entry_policy: SceneEntryPolicy,
    pub events: Vec<CanonicalEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEntryPolicy {
    pub stage: String,
    pub music: String,
    pub ambient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CanonicalEvent {
    #[serde(rename = "background")]
    Background { id: String, resource: String },

    #[serde(rename = "narration")]
    Narration { id: String, text: String },

    #[serde(rename = "dialogue")]
    Dialogue {
        id: String,
        speaker: String,
        text: String,
        voice: Option<String>,
    },

    #[serde(rename = "show")]
    Show {
        id: String,
        object: String,
        expression: Option<String>,
        position: Option<String>,
        transition: Option<String>,
        duration_ms: Option<u64>,
    },

    #[serde(rename = "hide")]
    Hide { id: String, object: String },

    #[serde(rename = "set-state")]
    SetState { id: String, path: String, value: crate::state::ScalarValue },

    #[serde(rename = "change-state")]
    ChangeState { id: String, path: String, by: f64 },

    #[serde(rename = "branch")]
    Branch {
        id: String,
        condition: crate::state::Condition,
        then: EventTarget,
        #[serde(rename = "else")]
        else_target: EventTarget,
    },

    #[serde(rename = "choice")]
    Choice {
        id: String,
        options: Vec<ChoiceOption>,
    },

    #[serde(rename = "jump")]
    Jump { id: String, target: EventTarget },

    #[serde(rename = "checkpoint")]
    Checkpoint { id: String, checkpoint_id: String, label: String },

    #[serde(rename = "end")]
    End { id: String, title: String, result: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub id: String,
    pub label: String,
    pub target: EventTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTarget {
    pub scene: Option<String>,
    pub event: Option<String>,
}
