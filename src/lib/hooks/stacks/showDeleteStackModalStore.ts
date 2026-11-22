import {writable, type Writable} from "svelte/store";

export const showDeleteStackModalStore: Writable<boolean> = writable(false);

export function toggleDeleteStackModal(visibility: boolean | null = null): void {
    if (visibility) showDeleteStackModalStore.set(visibility);
    else showDeleteStackModalStore.set(!showDeleteStackModalStore);
}