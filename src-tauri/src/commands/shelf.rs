use tauri::State;
use crate::models::shelf::Shelf;
use crate::repositories::shelf::ShelfRepository;

#[tauri::command]
pub fn read_all_shelves(repository: State<ShelfRepository>) -> Vec<Shelf> {
    let shelves: Vec<Shelf> = repository.read_all().unwrap();

    return shelves;
}

#[tauri::command]
pub fn read_one_shelf() {
    //
}

#[tauri::command]
pub fn create_shelf() {
    //
}

#[tauri::command]
pub fn update_shelf() {
    //
}

#[tauri::command]
pub fn delete_shelf() {
    //
}