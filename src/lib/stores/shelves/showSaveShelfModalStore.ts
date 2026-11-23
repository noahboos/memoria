import {writable, type Writable} from "svelte/store";

export const showSaveShelfModalStore: Writable<boolean> = writable(false);

export function toggleSaveShelfModal(visibility: boolean | null = null): void {
    if (visibility) showSaveShelfModalStore.set(visibility);
    else showSaveShelfModalStore.set(!showSaveShelfModalStore);
}