import {writable, type Writable} from "svelte/store";

export const showSaveStackModalStore : Writable<boolean> = writable(false);

export function toggleSaveStackModal(visibility: boolean | null = null): void {
    if (visibility) showSaveStackModalStore.set(visibility);
    else showSaveStackModalStore.set(!showSaveStackModalStore);
}