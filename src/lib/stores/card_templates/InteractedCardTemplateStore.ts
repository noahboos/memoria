import {writable, type Writable} from "svelte/store";
import {toggleDeleteCardTemplateModal} from "$lib/stores/card_templates/showDeleteCardTemplateModalStore";

export class InteractedCardTemplateStore {
    static idStore: Writable<number | null> = writable(null);

    static displayDeleteModal(id: number): void {
        this.idStore.set(id);
        toggleDeleteCardTemplateModal(true);
    }
}