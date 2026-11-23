<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import type {Shelf} from "$lib/types/Shelf";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import ListElement from "$lib/components/shelves/ListElement.svelte";
    import {showSaveShelfModalStore} from "$lib/stores/shelves/showSaveShelfModalStore";
    import SaveShelfModal from "$lib/components/shelves/SaveShelfModal.svelte";
    import {showDeleteShelfModalStore} from "$lib/stores/shelves/showDeleteShelfModalStore";
    import DeleteShelfModal from "$lib/components/shelves/DeleteShelfModal.svelte";
    import {showSaveStackModalStore} from "$lib/stores/stacks/showSaveStackModalStore";
    import SaveStackModal from "$lib/components/stacks/SaveStackModal.svelte";
    import type {Writable} from "svelte/store";
    import {InteractedShelfStore} from "$lib/stores/shelves/InteractedShelfStore";
    import {InteractedStackStore} from "$lib/stores/stacks/InteractedStackStore";

    let shelves: Shelf[] = $state([]);
    const interactedShelfIdStore: Writable<number | null> = InteractedShelfStore.idStore;

    let isStackWithoutShelfIdExisting: boolean = $state(false);

    onMount(async () => {
        await getCurrentWindow().setTitle("Liste des étagères | Memoria")

        shelves = await invoke('read_all_shelves');
        isStackWithoutShelfIdExisting = await invoke("is_stack_without_shelf_id_existing");
    });

    async function refresh(): Promise<void> {
        $interactedShelfIdStore = null;
        shelves = await invoke('read_all_shelves');
        isStackWithoutShelfIdExisting = await invoke("is_stack_without_shelf_id_existing");
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">Étagères</h1>
    <div class="space-y-2">
        {#if shelves.length > 0}
            {#each shelves as shelf}
                <ListElement
                    {shelf}
                    onEdit={() => InteractedShelfStore.displaySaveModal(shelf.id)}
                    onDelete={() => InteractedShelfStore.displayDeleteModal(shelf.id ?? 0)}
                />
            {/each}
            {#if isStackWithoutShelfIdExisting}
                <article class="flex flex-row justify-between bg-base-25 w-2xl mx-auto rounded-md hover:bg-accent-500/35 relative">
                    <a
                        class="py-2 px-4 max-w-fit underline after:content-[''] after:absolute after:inset-0"
                        href="{ "/shelves/" + 0 }"
                    >Piles flottantes</a>
                </article>
            {/if}
        {:else}
            <article class="flex flex-row justify-between py-2 px-4 bg-base-25 w-2xl mx-auto rounded-md relative">
                <p>Aucune étagère n'existe.</p>
            </article>
        {/if}
    </div>
    <div class="flex flex-row gap-2 justify-center">
        <button
            class="w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={() => InteractedShelfStore.displaySaveModal()}
        >Créer une nouvelle étagère</button>
        <button
            class="w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={() => InteractedStackStore.displaySaveModal()}
        >Créer une nouvelle pile</button>
    </div>
</main>
{#if $showSaveShelfModalStore}
    <SaveShelfModal id={$interactedShelfIdStore} onSave={refresh} />
{/if}
{#if $showDeleteShelfModalStore}
    <DeleteShelfModal id={$interactedShelfIdStore} onDelete={refresh} />
{/if}
{#if $showSaveStackModalStore}
    <SaveStackModal id={null} shelf_id={null} onSave={refresh} />
{/if}
