use std::fmt::format;
use tauri::State;
use crate::models::card_template::CardTemplate;
use crate::repositories::card_template::repository::CardTemplateRepository;

#[tauri::command]
pub fn read_all_card_templates(repository: State<CardTemplateRepository>) -> Vec<CardTemplate> {
    return repository.read_all().unwrap();
}

#[tauri::command]
pub fn read_all_card_templates_by_note_template_id(repository: State<CardTemplateRepository>, note_template_id: i64) -> Vec<CardTemplate> {
    return repository.read_all().unwrap().into_iter()
        .filter(|card_template| card_template.note_template_id.unwrap_or(0) == note_template_id)
        .collect();
}

#[tauri::command]
pub fn read_one_card_template(repository: State<CardTemplateRepository>, id: i64) -> CardTemplate {
    return repository.read_one(id).unwrap();
}

#[tauri::command]
pub fn save_card_template(repository: State<CardTemplateRepository>, card_template: CardTemplate) -> Result<CardTemplate, String> {
    if card_template.id.is_none() {
        return repository
            .create(&card_template)
            .map_err(|error| format!("Unable to create card template: {error:?}"));
    } else {
        return repository
            .update(&card_template)
            .map_err(|error| format!("Unable to update card template: {error:?}"));
    }
}

#[tauri::command]
pub fn delete_card_template(repository: State<CardTemplateRepository>, id: i64) -> Result<(), String> {
    return repository
        .delete(id)
        .map_err(|error| format!("Unable to delete card template: {error:?}"))
}