use crate::ipc::{ChoiceOptionDto, ManifestSummary, StepEventDto, StepResponse};
use crate::container::VnpPackage;
use crate::machine::RuntimeMachine;
use crate::schema::{CanonicalEvent, SceneDocument};
use crate::save::manager::SaveManager;
use crate::save::schema::{PresentationSnapshot, WorkIdentity};
use crate::state::StateRegistry;
use base64::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub package: Mutex<Option<VnpPackage<File>>>,
    pub machine: Mutex<Option<RuntimeMachine>>,
    pub snapshot: Mutex<PresentationSnapshot>,
    pub current_pending_kind: Mutex<Option<String>>,
    pub current_pending_id: Mutex<Option<String>>,
}

#[tauri::command]
pub fn load_package(
    package_path: String,
    state: State<AppState>,
) -> Result<ManifestSummary, String> {
    let file = File::open(&package_path).map_err(|e| e.to_string())?;
    let mut package = VnpPackage::open(file).map_err(|e| e.to_string())?;

    let summary = ManifestSummary {
        id: package.manifest.id.clone(),
        title: package.manifest.title.clone(),
        version: package.manifest.version.clone(),
        default_language: package.manifest.default_language.clone(),
        entry_scene: package.manifest.entry_scene.clone(),
    };

    // Extract scene paths first to avoid multiple borrow conflict
    let scene_entries: Vec<(String, String)> = package
        .scene_catalog
        .scenes
        .iter()
        .map(|(id, entry)| (id.clone(), entry.href.clone()))
        .collect();

    let mut scenes_map = HashMap::new();
    for (scene_id, href) in scene_entries {
        let scene_doc: SceneDocument = package
            .load_scene_json(&href)
            .map_err(|e| e.to_string())?;
        scenes_map.insert(scene_id, scene_doc);
    }

    let entry_scene = package.manifest.entry_scene.clone();
    let machine = RuntimeMachine::new(scenes_map, entry_scene, StateRegistry::default(), 42);

    *state.package.lock().unwrap() = Some(package);
    *state.machine.lock().unwrap() = Some(machine);
    *state.snapshot.lock().unwrap() = PresentationSnapshot::default();

    Ok(summary)
}

#[tauri::command]
pub fn advance_story(state: State<AppState>) -> Result<StepResponse, String> {
    let mut machine_guard = state.machine.lock().unwrap();
    let machine = machine_guard
        .as_mut()
        .ok_or_else(|| "No active game session loaded".to_string())?;

    let mut snapshot = state.snapshot.lock().unwrap();

    loop {
        let scene = machine
            .scenes
            .get(&machine.current_scene_id)
            .ok_or_else(|| "Current scene missing".to_string())?;

        if machine.current_event_index >= scene.events.len() {
            return Err("Execution dropped off event array without valid exit".into());
        }

        let event = &scene.events[machine.current_event_index];

        match event {
            CanonicalEvent::Background { id: _, resource } => {
                snapshot.background = Some(resource.clone());
                machine.current_event_index += 1;
            }
            CanonicalEvent::Narration { id, text } => {
                machine.current_event_index += 1;
                *state.current_pending_kind.lock().unwrap() = Some("advance".into());
                *state.current_pending_id.lock().unwrap() = Some(id.clone());

                return Ok(StepResponse {
                    event: StepEventDto::Narration {
                        beat_id: id.clone(),
                        text: text.clone(),
                    },
                    presentation: snapshot.clone(),
                });
            }
            CanonicalEvent::Dialogue {
                id,
                speaker,
                text,
                voice,
            } => {
                machine.current_event_index += 1;
                *state.current_pending_kind.lock().unwrap() = Some("advance".into());
                *state.current_pending_id.lock().unwrap() = Some(id.clone());

                return Ok(StepResponse {
                    event: StepEventDto::Dialogue {
                        beat_id: id.clone(),
                        speaker: speaker.clone(),
                        speaker_display: speaker.clone(),
                        text: text.clone(),
                        voice_resource: voice.clone(),
                    },
                    presentation: snapshot.clone(),
                });
            }
            CanonicalEvent::Choice { id, options } => {
                *state.current_pending_kind.lock().unwrap() = Some("choice".into());
                *state.current_pending_id.lock().unwrap() = Some(id.clone());

                let option_dtos = options
                    .iter()
                    .map(|o| ChoiceOptionDto {
                        id: o.id.clone(),
                        label: o.label.clone(),
                    })
                    .collect();

                return Ok(StepResponse {
                    event: StepEventDto::Choice {
                        choice_id: id.clone(),
                        options: option_dtos,
                    },
                    presentation: snapshot.clone(),
                });
            }
            CanonicalEvent::End { id: _, title, result } => {
                return Ok(StepResponse {
                    event: StepEventDto::Ended {
                        title: title.clone(),
                        result: result.clone(),
                    },
                    presentation: snapshot.clone(),
                });
            }
            _ => {
                machine.advance().map_err(|e| e.to_string())?;
            }
        }
    }
}

#[tauri::command]
pub fn choose_option(option_id: String, state: State<AppState>) -> Result<StepResponse, String> {
    let mut machine_guard = state.machine.lock().unwrap();
    let machine = machine_guard
        .as_mut()
        .ok_or_else(|| "No active game session loaded".to_string())?;

    let scene = machine
        .scenes
        .get(&machine.current_scene_id)
        .ok_or_else(|| "Current scene missing".to_string())?;

    let event = &scene.events[machine.current_event_index];

    if let CanonicalEvent::Choice { options, .. } = event {
        let opt = options
            .iter()
            .find(|o| o.id == option_id)
            .ok_or_else(|| format!("Option ID {} not valid in current choice", option_id))?;

        let target = opt.target.clone();
        drop(machine_guard);

        let mut machine_guard = state.machine.lock().unwrap();
        let machine = machine_guard.as_mut().unwrap();

        if let Some(target_scene) = target.scene {
            machine.current_scene_id = target_scene;
            machine.current_event_index = 0;
        } else if let Some(target_event) = target.event {
            let target_idx = machine
                .scenes
                .get(&machine.current_scene_id)
                .unwrap()
                .events
                .iter()
                .position(|e| machine.extract_event_id(e) == target_event)
                .ok_or_else(|| "Target jump event not found".to_string())?;
            machine.current_event_index = target_idx;
        }

        drop(machine_guard);
        advance_story(state)
    } else {
        Err("Current event is not a Choice beat".into())
    }
}

#[tauri::command]
pub fn export_save(state: State<AppState>) -> Result<String, String> {
    let machine_guard = state.machine.lock().unwrap();
    let machine = machine_guard.as_ref().ok_or_else(|| "No active machine".to_string())?;

    let package_guard = state.package.lock().unwrap();
    let package = package_guard.as_ref().ok_or_else(|| "No package loaded".to_string())?;

    let snapshot = state.snapshot.lock().unwrap().clone();
    let pending_kind = state.current_pending_kind.lock().unwrap().clone();
    let pending_id = state.current_pending_id.lock().unwrap().clone();

    let work = WorkIdentity {
        id: package.manifest.id.clone(),
        version: package.manifest.version.clone(),
        compatibility_id: format!("{}:1", package.manifest.id),
    };

    SaveManager::create_save(
        machine,
        work,
        package.manifest.default_language.clone(),
        "2026-08-21T02:30:00Z".to_string(),
        snapshot,
        pending_id,
        pending_kind,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_save(save_json: String, state: State<AppState>) -> Result<PresentationSnapshot, String> {
    let mut machine_guard = state.machine.lock().unwrap();
    let machine = machine_guard.as_mut().ok_or_else(|| "No active machine".to_string())?;

    let package_guard = state.package.lock().unwrap();
    let package = package_guard.as_ref().ok_or_else(|| "No package loaded".to_string())?;

    let compat_id = format!("{}:1", package.manifest.id);

    let restored_snapshot = SaveManager::restore_save(
        &save_json,
        machine,
        &package.manifest.id,
        &compat_id,
    )
    .map_err(|e| e.to_string())?;

    *state.snapshot.lock().unwrap() = restored_snapshot.clone();
    Ok(restored_snapshot)
}

#[tauri::command]
pub fn get_resource_base64(path: String, state: State<AppState>) -> Result<String, String> {
    let mut package_guard = state.package.lock().unwrap();
    let package = package_guard.as_mut().ok_or_else(|| "No package loaded".to_string())?;

    let bytes = package
        .read_verified_resource(&path)
        .map_err(|e| e.to_string())?;
    Ok(BASE64_STANDARD.encode(bytes))
}
