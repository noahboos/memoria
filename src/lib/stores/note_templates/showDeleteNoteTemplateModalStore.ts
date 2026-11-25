import {writable, type Writable} from "svelte/store";

export const showDeleteNoteTemplateModalStore: Writable<boolean> = writable(false);

export function toggleDeleteNoteTemplateModal(visibility: boolean | null = null): void {
    if (visibility) showDeleteNoteTemplateModalStore.set(visibility);
    else showDeleteNoteTemplateModalStore.set(!showDeleteNoteTemplateModalStore);
}