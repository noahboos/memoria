use std::sync::Arc;
use tauri::{App, Manager};
use crate::database::Database;
use crate::repositories::card_template::repository::CardTemplateRepository;
use crate::repositories::note_template::repository::NoteTemplateRepository;
use crate::repositories::shelf::ShelfRepository;
use crate::repositories::stack::StackRepository;

mod algorithms;
mod commands;
mod database;
mod models;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app: &mut App| {
            let database: Database = Database::create(&app)?;
            let database: Arc<Database> = Arc::new(database);
            app.manage(database.clone());

            let card_template_repository: CardTemplateRepository = CardTemplateRepository::new(database.clone());
            app.manage(card_template_repository);

            let note_template_repository: NoteTemplateRepository = NoteTemplateRepository::new(database.clone());
            app.manage(note_template_repository);

            let shelf_repository: ShelfRepository = ShelfRepository::new(database.clone());
            app.manage(shelf_repository);

            let stack_repository: StackRepository = StackRepository::new(database.clone());
            app.manage(stack_repository);

            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            commands::read_all_shelves,
            commands::read_one_shelf,
            commands::save_shelf,
            commands::delete_shelf,
            commands::read_all_stacks,
            commands::read_all_stacks_by_shelf_id,
            commands::read_all_stacks_without_shelf_id,
            commands::is_stack_without_shelf_id_existing,
            commands::read_one_stack,
            commands::save_stack,
            commands::delete_stack
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
