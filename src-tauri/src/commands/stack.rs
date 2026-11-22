use tauri::State;
use crate::models::stack::Stack;
use crate::repositories::stack::StackRepository;

#[tauri::command]
pub fn read_all_stacks(repository: State<StackRepository>) -> Vec<Stack> {
    let stacks: Vec<Stack> = repository.read_all().unwrap();

    return stacks;
}

#[tauri::command]
pub fn read_one_stack(repository: State<StackRepository>, id: i64) -> Stack {
    let stack: Stack = repository.read_one(id).unwrap();

    return stack;
}

#[tauri::command]
pub fn save_stack(repository: State<StackRepository>, stack: Stack) -> Result<Stack, String> {
    if stack.id.is_none() {
        return repository
            .create(&stack)
            .map_err(|error| format!("Unable to create the stack : {error:?}"));
    } else {
        return repository
            .update(&stack)
            .map_err(|error| format!("Unable to update the stack : {error:?}"));
    }
}

#[tauri::command]
pub fn delete_stack(repository: State<StackRepository>, id: i64) -> Result<(), String> {
    return repository
        .delete(id)
        .map_err(|error| format!("Unable to delete the stack : {error:?}"));
}