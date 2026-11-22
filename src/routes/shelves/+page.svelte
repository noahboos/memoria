<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import type {Shelf} from "$lib/types/Shelf";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import ListElement from "$lib/components/shelves/ListElement.svelte";
    import {showSaveShelfModalStore, toggleSaveShelfModal} from "$lib/hooks/shelves/showSaveShelfModalStore";
    import SaveShelfModal from "$lib/components/shelves/SaveShelfModal.svelte";
    import {showDeleteShelfModalStore, toggleDeleteShelfModal} from "$lib/hooks/shelves/showDeleteShelfModalStore";
    import DeleteShelfModal from "$lib/components/shelves/DeleteShelfModal.svelte";

    let shelves: Shelf[] = $state([]);
    let shelfId: number | null = $state(null);

    onMount(async () => {
        await getCurrentWindow().setTitle("Liste des étagères | Memoria")

        shelves = await invoke('read_all_shelves');
        console.log(shelves);
    });

    function createShelf(): void {
        shelfId = null;
        toggleSaveShelfModal(true);
    }

    function editShelf(id:number): void {
        shelfId = id;
        toggleSaveShelfModal(true);
    }

    function deleteShelf(id:number): void {
        shelfId = id;
        toggleDeleteShelfModal(true);
    }

    async function refresh(): Promise<void> {
        shelfId = null;
        shelves = await invoke('read_all_shelves');
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">Étagères</h1>
    <div class="space-y-2">
        {#if shelves.length > 0}
            {#each shelves as shelf}
                <ListElement {shelf} onEdit={editShelf} onDelete={deleteShelf} />
            {/each}
        {:else}
            <article class="flex flex-row justify-between py-2 px-4 bg-base-25 w-2xl mx-auto rounded-md relative">
                <p>Aucune étagère n'existe.</p>
            </article>
        {/if}
    </div>
    <button
        class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
        onclick={createShelf}
    >Créer une nouvelle étagère</button>
</main>
{#if $showSaveShelfModalStore}
    <SaveShelfModal id={shelfId} onSave={refresh} />
{/if}
{#if $showDeleteShelfModalStore}
    <DeleteShelfModal id={shelfId} onDelete={refresh} />
{/if}
