<script lang="ts">
    import type {Stack} from "$lib/types/Stack";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {toggleSaveStackModal} from "$lib/stores/stacks/showSaveStackModalStore";
    import {X} from "@lucide/svelte";
    import type {Shelf} from "$lib/types/Shelf";
    import {refreshIsStackWithoutShelfIdExistingStore} from "$lib/stores/stacks/isStackWithoutShelfIdExistingStore";
    import {get} from "svelte/store";
    import {InteractedStackStore} from "$lib/stores/stacks/InteractedStackStore";

    interface Props {
        onSave: () => Promise<void>,
    }

    let {onSave}: Props = $props()

    let id: number | null = get(InteractedStackStore.idStore);

    let shelves: Shelf[] = $state([]);
    let stack: Stack = $state({ id, shelf_id: null, name: null, algorithm: "SM2", created_at: null, modified_at: null });

    onMount(async () => {
        shelves = await invoke("read_all_shelves");
        if (id !== null) {
            stack = await invoke("read_one_stack", {id});
        }
    });

    function close(): void {
        toggleSaveStackModal(false);
    }

    async function save(): Promise<void> {
        await invoke("save_stack", { stack });
        toggleSaveStackModal(false);
        await refreshIsStackWithoutShelfIdExistingStore();
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
                    Créer une nouvelle pile
                {:else}
                    Mettre à jour « {stack.name} »
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
                    for="shelf_id"
                >Étagère</label>
                <select
                    class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
                    id="shelf_id"
                    bind:value={stack.shelf_id}
                >
                    <option value={null}>Aucune étagère</option>
                    {#each shelves as shelf}
                        <option value={shelf.id}>{shelf.name}</option>
                    {/each}
                </select>
            </div>
            <div class="flex flex-col gap-2">
                <label
                    class="font-bold"
                    for="name"
                >Nom</label>
                <input
                    class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
                    id="name"
                    type="text"
                    bind:value={stack.name}
                    placeholder="Exemple : « Cours d'histoire - Chapitre IV »"
                    required
                />
            </div>
            <div class="flex flex-col gap-2">
                <label
                    class="font-bold"
                    for="algorithm"
                >Algorithme</label>
                <select
                    class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
                    id="algorithm"
                    bind:value={stack.algorithm}
                >
                    <option value="SM2">SM2</option>
                    <option value="FSRS">FSRS</option>
                </select>
            </div>
            <button
                class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
                type="submit"
            >Enregistrer</button>
        </form>
    </div>
</div>