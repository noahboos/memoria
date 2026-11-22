<script lang="ts">
    import type {Shelf} from "$lib/types/Shelf";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {toggleSaveShelfModal} from "$lib/hooks/shelves/showSaveShelfModalStore";
    import { X } from "@lucide/svelte";

    interface Props {
        id: number | null,
        onSave: () => Promise<void>,
    }

    let {id, onSave}: Props = $props();

    let shelf: Shelf = $state({ id: null, name: null, created_at: null, modified_at: null });

    onMount(async () => {
        if (id !== null) shelf = await invoke("read_one_shelf", { id });
    });

    function close(): void {
        console.log("close");
        toggleSaveShelfModal(false);
    }

    async function save(): Promise<void> {
        await invoke("save_shelf", { shelf });
        toggleSaveShelfModal(false);
        await onSave();
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    onclick={close}
>
    <div
        class="space-y-4 w-xl h-fit p-4 border-transparent rounded-md shadow-md bg-base-00"
        onclick={(event) => event.stopPropagation()}
    >
        <div class="relative">
            <p class="font-serif text-lg text-center">
                {#if id === null}
                    Créer une nouvelle étagère
                {:else}
                    Mettre à jour « {shelf.name} »
                {/if}
            </p>
            <button
                class="absolute top-1 right-0 hover:text-red-500"
                onclick={close}
            ><X size="20" /></button>
        </div>
        <form class="space-y-4" onsubmit={save}>
            <div class="flex flex-col gap-2">
                <label
                    class="font-bold"
                    for="name"
                >Nom</label>
                <input
                    class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 "
                    id="name"
                    type="text"
                    bind:value={shelf.name}
                    placeholder="Exemple : « Mon étagère de polonais »"
                    required
                />
            </div>
            <button
                class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
                type="submit"
            >Enregistrer</button>
        </form>
    </div>
</div>