pub mod commands;
pub mod container;
pub mod ipc;
pub mod machine;
pub mod prng;
pub mod save;
pub mod schema;
pub mod state;

use commands::*;
use std::sync::Mutex;
use save::schema::PresentationSnapshot;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState {
            package: Mutex::new(None),
            machine: Mutex::new(None),
            snapshot: Mutex::new(PresentationSnapshot::default()),
            current_pending_kind: Mutex::new(None),
            current_pending_id: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            load_package,
            advance_story,
            choose_option,
            export_save,
            import_save,
            get_resource_base64
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}