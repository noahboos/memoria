use tauri::State;
use crate::models::note_template::NoteTemplate;
use crate::repositories::note_template::repository::NoteTemplateRepository;

#[tauri::command]
pub fn read_all_note_templates(repository: State<NoteTemplateRepository>) -> Vec<NoteTemplate> {
    return repository.read_all().unwrap();
}

#[tauri::command]
pub fn read_one_note_template(repository: State<NoteTemplateRepository>, id: i64) -> NoteTemplate {
    return repository.read_one(id).unwrap();
}

#[tauri::command]
pub fn save_note_template(repository: State<NoteTemplateRepository>, note_template: NoteTemplate) -> Result<NoteTemplate, String> {
    if note_template.id.is_none() {
        return repository
            .create(&note_template)
            .map_err(|error| format!("Unable to create note template: {error:?}"));
    } else {
        return repository
            .update(&note_template)
            .map_err(|error| format!("Unable to update note template: {error:?}"));
    }
}

#[tauri::command]
pub fn delete_note_template(repository: State<NoteTemplateRepository>, id: i64) -> Result<(), String> {
    return repository
        .delete(id)
        .map_err(|error| format!("Unable to delete note template: {error:?}"));
}