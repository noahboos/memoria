import {writable, type Writable} from "svelte/store";
import {toggleSaveStackModal} from "$lib/stores/stacks/showSaveStackModalStore";
import {toggleDeleteStackModal} from "$lib/stores/stacks/showDeleteStackModalStore";

export class InteractedStackStore {
    static idStore: Writable<number | null> = writable(null);

    static displaySaveModal(id: number | null = null): void {
        this.idStore.set(id);
        toggleSaveStackModal(true);
    }

    static displayDeleteModal(id: number): void {
        this.idStore.set(id);
        toggleDeleteStackModal(true);
    }
}

