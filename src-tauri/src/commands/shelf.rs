use std::fmt::format;
use tauri::State;
use crate::models::shelf::Shelf;
use crate::repositories::shelf::{ShelfRepository, ShelfRepositoryError};

#[tauri::command]
pub fn read_all_shelves(repository: State<ShelfRepository>) -> Vec<Shelf> {
    let shelves: Vec<Shelf> = repository.read_all().unwrap();

    return shelves;
}

#[tauri::command]
pub fn read_one_shelf(repository: State<ShelfRepository>, id: i64) -> Shelf {
    let shelf: Shelf = repository.read_one(id).unwrap();

    return shelf;
}

#[tauri::command]
pub fn save_shelf(repository: State<ShelfRepository>, shelf: Shelf) -> Result<Shelf, String> {
    if shelf.id.is_none() {
        let name = shelf
            .name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .ok_or_else(|| "Shelf's name is a mandatory attribute.".to_string())?;

        return repository
            .create(name)
            .map_err(|error| format!("Unable to create the shelf : {error:?}"));
    } else {
        return repository
            .update(&shelf)
            .map_err(|error| format!("Unable to update the shelf : {error:?}"));
    }
}

#[tauri::command]
pub fn delete_shelf(repository: State<ShelfRepository>, id: i64) -> Result<(), String> {
    return repository
        .delete(id)
        .map_err(|error| format!("Unable to delete the shelf : {error:?}"));
}