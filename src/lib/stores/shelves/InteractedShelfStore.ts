import {writable, type Writable} from "svelte/store";
import {toggleSaveShelfModal} from "$lib/stores/shelves/showSaveShelfModalStore";
import {toggleDeleteShelfModal} from "$lib/stores/shelves/showDeleteShelfModalStore";

export class InteractedShelfStore {
    static idStore: Writable<number | null> = writable(null);

    static displaySaveModal(id: number | null = null): void {
        this.idStore.set(id);
        toggleSaveShelfModal(true);
    }

    static displayDeleteModal(id: number): void {
        this.idStore.set(id);
        toggleDeleteShelfModal(true);
    }
}