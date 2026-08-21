#[cfg(test)]
mod tests {
    use super::manager::*;
    use super::schema::*;
    use crate::machine::RuntimeMachine;
    use crate::schema::*;
    use crate::state::{ScalarValue, StateRegistry};
    use std::collections::HashMap;

    #[test]
    fn test_save_and_restore_cycle() {
        // Setup minimal scene[cite: 2]
        let mut scenes = HashMap::new();
        scenes.insert(
            "station-arrival".to_string(),
            SceneDocument {
                id: "station-arrival".to_string(),
                mode: "adv".to_string(),
                entry_event: "narration-1".to_string(),
                entry_policy: SceneEntryPolicy {
                    stage: "reset".to_string(),
                    music: "continue".to_string(),
                    ambient: "replace".to_string(),
                },
                events: vec![
                    CanonicalEvent::Narration {
                        id: "narration-1".to_string(),
                        text: "The last train was due in seven minutes.".to_string(),
                    },
                    CanonicalEvent::Dialogue {
                        id: "maya-line-1".to_string(),
                        speaker: "maya".to_string(),
                        text: "Daniel? Are you here?".to_string(),
                        voice: None,
                    },
                ],
            },
        );

        let mut initial_state = StateRegistry::default();
        initial_state.set("has-letter".to_string(), ScalarValue::Boolean(true));
        initial_state.set("trust".to_string(), ScalarValue::Number(2.0));

        let mut machine = RuntimeMachine::new(
            scenes.clone(),
            "station-arrival".to_string(),
            initial_state,
            1337,
        );

        // Advance past first narration beat[cite: 3, 6]
        let _ = machine.advance().unwrap();

        let work = WorkIdentity {
            id: "com.example.the-last-train".to_string(),
            version: "1.0.0".to_string(),
            compatibility_id: "com.example.the-last-train:1".to_string(),
        };

        let mut snapshot = PresentationSnapshot::default();
        snapshot.background = Some("platform-night".to_string());
        snapshot.objects.push(StagedObject {
            id: "maya".to_string(),
            expression: Some("worried".to_string()),
            position: Some("left".to_string()),
            layer: "stage".to_string(),
            scale: 1.0,
            opacity: 1.0,
        });

        // 1. Serialize Save[cite: 6]
        let save_json = SaveManager::create_save(
            &machine,
            work.clone(),
            "en".to_string(),
            "2026-08-21T01:51:25Z".to_string(),
            snapshot.clone(),
            Some("maya-line-1".to_string()),
            Some("advance".to_string()),
        ).unwrap();

        // 2. Restore into a new machine[cite: 6]
        let mut restored_machine = RuntimeMachine::new(
            scenes,
            "station-arrival".to_string(),
            StateRegistry::default(),
            1, // Arbitrary seed to be overwritten[cite: 6]
        );

        let restored_snapshot = SaveManager::restore_save(
            &save_json,
            &mut restored_machine,
            "com.example.the-last-train",
            "com.example.the-last-train:1",
        ).unwrap();

        // Assertions[cite: 6]
        assert_eq!(restored_machine.current_scene_id, "station-arrival");
        assert_eq!(restored_machine.current_event_index, 1);
        assert_eq!(restored_machine.prng.state, 1337);
        assert_eq!(
            restored_machine.run_state.get("has-letter"),
            Some(&ScalarValue::Boolean(true))
        );
        assert_eq!(
            restored_machine.run_state.get("trust"),
            Some(&ScalarValue::Number(2.0))
        );
        assert_eq!(restored_snapshot.background, Some("platform-night".to_string()));
        assert_eq!(restored_snapshot.objects[0].id, "maya");
    }
}
