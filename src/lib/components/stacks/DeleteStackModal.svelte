<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import type {Stack} from "$lib/types/Stack";
    import { X } from "@lucide/svelte";
    import {toggleDeleteStackModal} from "$lib/stores/stacks/showDeleteStackModalStore";
    import {refreshIsStackWithoutShelfIdExistingStore} from "$lib/stores/stacks/isStackWithoutShelfIdExistingStore";

    interface Props {
        id: number | null,
        onDelete: () => Promise<void>,
    }

    let {id, onDelete}: Props = $props();

    let stack: Stack = $state({ id: null, shelf_id: null, name: null, algorithm: null, created_at: null, modified_at: null });

    onMount(async() => {
        if (id !== null) stack = await invoke("read_one_stack", {id})
    });

    function close(): void {
        toggleDeleteStackModal(false);
    }

    async function trash(): Promise<void> {
        await invoke("delete_stack", {id});
        toggleDeleteStackModal(false);
        await refreshIsStackWithoutShelfIdExistingStore();
        await onDelete();
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    onclick={close}
>
    <div
        class="space-y-4 w-xl h-fit p-4 border-transparent rounded-md shadod-md bg-base-00"
        onclick={(event) => event.stopPropagation()}
    >
        <div class="relative">
            <p
                class="font-serif text-lg text-center"
            >Supprimer « {stack.name} »</p>
            <button
                class="absolute top-1 right-0 hover:text-red-500"
                onclick={trash}
            ><X size="20" /></button>
        </div>
        <p>En supprimant une pile, l'entièreté de ses notes et de ses cartes seront supprimées. Soyez sûr de ne plus en avoir besoin, lorsque vous cliquez sur le bouton de suppression.</p>
        <button
            class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            type="button"
            onclick={trash}
        >Supprimer</button>
    </div>
</div>