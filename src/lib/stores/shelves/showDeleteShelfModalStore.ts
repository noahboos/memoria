import {writable, type Writable} from "svelte/store";

export const showDeleteShelfModalStore: Writable<boolean> = writable(false);

export function toggleDeleteShelfModal(visibility: boolean | null = null): void {
    if (visibility) showDeleteShelfModalStore.set(visibility);
    else showDeleteShelfModalStore.set(!showDeleteShelfModalStore);
}