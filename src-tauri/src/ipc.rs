use serde::{Deserialize, Serialize};
use crate::save::schema::PresentationSnapshot;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSummary {
    pub id: String,
    pub title: String,
    pub version: String,
    pub default_language: String,
    pub entry_scene: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceOptionDto {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum StepEventDto {
    Narration {
        beat_id: String,
        text: String,
    },
    Dialogue {
        beat_id: String,
        speaker: String,
        speaker_display: String,
        text: String,
        voice_resource: Option<String>,
    },
    Choice {
        choice_id: String,
        options: Vec<ChoiceOptionDto>,
    },
    Ended {
        title: String,
        result: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepResponse {
    pub event: StepEventDto,
    pub presentation: PresentationSnapshot,
}
