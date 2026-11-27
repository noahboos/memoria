import {writable, type Writable} from "svelte/store";

export const showDeleteCardTemplateModalStore: Writable<boolean> = writable(false);

export function toggleDeleteCardTemplateModal(visibility: boolean | null = null): void {
    if (visibility) showDeleteCardTemplateModalStore.set(visibility);
    else showDeleteCardTemplateModalStore.set(!showDeleteCardTemplateModalStore);
}