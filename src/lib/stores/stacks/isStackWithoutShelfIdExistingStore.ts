import {writable, type Writable} from "svelte/store";
import {invoke} from "@tauri-apps/api/core";

export const isStackWithoutShelfIdExistingStore: Writable<boolean> = writable(false);

export async function refreshIsStackWithoutShelfIdExistingStore(): Promise<void> {
    isStackWithoutShelfIdExistingStore.set(await invoke("is_stack_without_shelf_id_existing"));
}