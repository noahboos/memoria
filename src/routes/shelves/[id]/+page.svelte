<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {Shelf} from "$lib/types/Shelf";
    import type {PageProps} from "../../../../.svelte-kit/types/src/routes/shelves/[id]/$types";
    import {showDeleteShelfModalStore, toggleDeleteShelfModal} from "$lib/hooks/shelves/showDeleteShelfModalStore";
    import DeleteShelfModal from "$lib/components/shelves/DeleteShelfModal.svelte";
    import {showSaveShelfModalStore, toggleSaveShelfModal} from "$lib/hooks/shelves/showSaveShelfModalStore";
    import SaveShelfModal from "$lib/components/shelves/SaveShelfModal.svelte";
    import {goto} from "$app/navigation";

    let {params}: PageProps = $props();

    const id: number = Number(params.id);
    let shelf: Shelf = $state({ id: null, name: null, created_at: null, modified_at: null });

    onMount(async () => {
        await refresh();
    });

    function editShelf(): void {
        toggleSaveShelfModal(true);
    }

    function deleteShelf(): void {
        toggleDeleteShelfModal(true);
    }

    async function refresh(): Promise<void> {
        shelf = await invoke("read_one_shelf", {id});
    }

    async function onDelete(): Promise<void> {
        await goto("/shelves/");
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">{shelf.name}</h1>
    <div class="flex flex-row gap-2 justify-center">
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={editShelf}
        >Modifier</button>
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={deleteShelf}
        >Supprimer</button>
    </div>
</main>
{#if $showSaveShelfModalStore}
    <SaveShelfModal {id} onSave={refresh} />
{/if}
{#if $showDeleteShelfModalStore}
    <DeleteShelfModal {id} onDelete={onDelete} />
{/if}