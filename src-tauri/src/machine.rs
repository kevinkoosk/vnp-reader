use crate::prng::VnpPrng1;
use crate::schema::{CanonicalEvent, EventTarget, SceneDocument};
use crate::state::StateRegistry;
use std::collections::HashMap;

pub enum StepResult {
    AwaitingAdvance {
        beat_id: String,
        text: String,
        speaker: Option<String>,
    },
    AwaitingChoice {
        choice_id: String,
        options: Vec<(String, String)>,
    },
    Ended {
        title: String,
        result: String,
    },
    SceneChanged {
        new_scene_id: String,
    },
}

pub struct RuntimeMachine {
    pub scenes: HashMap<String, SceneDocument>,
    pub current_scene_id: String,
    pub current_event_index: usize,
    pub run_state: StateRegistry,
    pub prng: VnpPrng1,
    pub call_stack: Vec<EventTarget>,
}

impl RuntimeMachine {
    pub fn new(
        scenes: HashMap<String, SceneDocument>,
        entry_scene: String,
        initial_state: StateRegistry,
        seed: u32,
    ) -> Self {
        Self {
            scenes,
            current_scene_id: entry_scene,
            current_event_index: 0,
            run_state: initial_state,
            prng: VnpPrng1::new(seed).unwrap(),
            call_stack: Vec::new(),
        }
    }

    pub fn extract_event_id(&self, event: &CanonicalEvent) -> String {
        match event {
            CanonicalEvent::Background { id, .. }
            | CanonicalEvent::Narration { id, .. }
            | CanonicalEvent::Dialogue { id, .. }
            | CanonicalEvent::Show { id, .. }
            | CanonicalEvent::Hide { id, .. }
            | CanonicalEvent::SetState { id, .. }
            | CanonicalEvent::ChangeState { id, .. }
            | CanonicalEvent::Branch { id, .. }
            | CanonicalEvent::Choice { id, .. }
            | CanonicalEvent::Jump { id, .. }
            | CanonicalEvent::Checkpoint { id, .. }
            | CanonicalEvent::End { id, .. } => id.clone(),
        }
    }

    pub fn get_current_event_id(&self) -> String {
        let scene = self
            .scenes
            .get(&self.current_scene_id)
            .expect("Current scene missing");
        if self.current_event_index < scene.events.len() {
            self.extract_event_id(&scene.events[self.current_event_index])
        } else {
            "end-of-scene".to_string()
        }
    }

    pub fn advance(&mut self) -> Result<StepResult, String> {
        loop {
            let event = {
                let scene = self
                    .scenes
                    .get(&self.current_scene_id)
                    .ok_or_else(|| format!("Scene not found: {}", self.current_scene_id))?;

                if self.current_event_index >= scene.events.len() {
                    return Err("Execution reached the end of the scene event array without an exit.".into());
                }

                scene.events[self.current_event_index].clone()
            };

            match event {
                CanonicalEvent::Narration { id, text } => {
                    self.current_event_index += 1;
                    return Ok(StepResult::AwaitingAdvance {
                        beat_id: id,
                        text,
                        speaker: None,
                    });
                }
                CanonicalEvent::Dialogue {
                    id,
                    speaker,
                    text,
                    ..
                } => {
                    self.current_event_index += 1;
                    return Ok(StepResult::AwaitingAdvance {
                        beat_id: id,
                        text,
                        speaker: Some(speaker),
                    });
                }
                CanonicalEvent::Choice { id, options } => {
                    let option_tuples = options
                        .into_iter()
                        .map(|opt| (opt.id, opt.label))
                        .collect();
                    return Ok(StepResult::AwaitingChoice {
                        choice_id: id,
                        options: option_tuples,
                    });
                }
                CanonicalEvent::SetState { path, value, .. } => {
                    self.run_state.set(path, value);
                    self.current_event_index += 1;
                }
                CanonicalEvent::ChangeState { path, by, .. } => {
                    self.run_state.change_number(&path, by)?;
                    self.current_event_index += 1;
                }
                CanonicalEvent::Branch {
                    condition,
                    then,
                    else_target,
                    ..
                } => {
                    let target = if self.run_state.evaluate_condition(&condition) {
                        then
                    } else {
                        else_target
                    };
                    self.transfer(&target)?;
                }
                CanonicalEvent::Jump { target, .. } => {
                    self.transfer(&target)?;
                }
                CanonicalEvent::End { title, result, .. } => {
                    return Ok(StepResult::Ended { title, result });
                }
                _ => {
                    self.current_event_index += 1;
                }
            }
        }
    }

    fn transfer(&mut self, target: &EventTarget) -> Result<(), String> {
        if let Some(scene_id) = &target.scene {
            self.current_scene_id = scene_id.clone();
            self.current_event_index = 0;
        } else if let Some(event_id) = &target.event {
            let scene = self
                .scenes
                .get(&self.current_scene_id)
                .ok_or_else(|| "Current scene missing during local jump.".to_string())?;
            let idx = scene
                .events
                .iter()
                .position(|e| self.extract_event_id(e) == *event_id)
                .ok_or_else(|| format!("Target event ID not found: {}", event_id))?;

            self.current_event_index = idx;
        }
        Ok(())
    }
}
