import {writable, type Writable} from "svelte/store";
import {toggleDeleteNoteTemplateModal} from "$lib/stores/note_templates/showDeleteNoteTemplateModalStore";

export class InteractedNoteTemplateStore {
    static idStore: Writable<number | null> = writable(null);

    static displayDeleteModal(id: number): void {
        this.idStore.set(id);
        toggleDeleteNoteTemplateModal(true);
    }
}