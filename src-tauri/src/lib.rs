use tauri::{App, Manager};
use crate::database::Database;

mod database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app: &mut App| {
            let database: Database = database::Database::create(&app)?;
            app.manage(database);
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
