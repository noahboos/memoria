<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {Shelf} from "$lib/types/Shelf";
    import type {PageProps} from "../../../../.svelte-kit/types/src/routes/shelves/[id]/$types";
    import {showDeleteShelfModalStore} from "$lib/stores/shelves/showDeleteShelfModalStore";
    import DeleteShelfModal from "$lib/components/shelves/DeleteShelfModal.svelte";
    import {showSaveShelfModalStore} from "$lib/stores/shelves/showSaveShelfModalStore";
    import SaveShelfModal from "$lib/components/shelves/SaveShelfModal.svelte";
    import {goto} from "$app/navigation";
    import type {Stack} from "$lib/types/Stack";
    import {showSaveStackModalStore} from "$lib/stores/stacks/showSaveStackModalStore";
    import SaveStackModal from "$lib/components/stacks/SaveStackModal.svelte";
    import ListElement from "$lib/components/stacks/ListElement.svelte";
    import {showDeleteStackModalStore} from "$lib/stores/stacks/showDeleteStackModalStore";
    import DeleteStackModal from "$lib/components/stacks/DeleteStackModal.svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {InteractedStackStore} from "$lib/stores/stacks/InteractedStackStore";
    import {InteractedShelfStore} from "$lib/stores/shelves/InteractedShelfStore";

    let {params}: PageProps = $props();

    const id: number = Number(params.id);
    let shelf: Shelf = $state({ id: null, name: null, created_at: null, modified_at: null });
    let stacks: Stack[] = $state([]);

    onMount(async () => {
        await refresh();

        if (id > 0) await getCurrentWindow().setTitle("Détail de l'étagère « " + shelf.name + " » | Memoria");
        else await getCurrentWindow().setTitle("Détail des piles flottantes | Memoria");
    });

    async function refresh(): Promise<void> {
        if (id > 0) {
            shelf = await invoke("read_one_shelf", {id});
            stacks = await invoke("read_all_stacks_by_shelf_id", {shelfId: id});
        } else {
            stacks = await invoke("read_all_stacks_without_shelf_id");
        }
    }

    async function onDelete(): Promise<void> {
        await goto("/shelves/");
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">{shelf.name ?? "Piles flottantes"}</h1>
    {#if id > 0}
        <div class="flex flex-row gap-2 justify-center">
            <button
                class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
                onclick={() => InteractedShelfStore.displaySaveModal(id)}
            >Modifier</button>
            <button
                class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
                onclick={() => InteractedShelfStore.displayDeleteModal(id)}
            >Supprimer</button>
        </div>
    {/if}
    <h2 class="font-serif text-xl text-center">Piles</h2>
    <div class="space-y-2">
        {#if stacks.length > 0}
            {#each stacks as stack}
                <ListElement
                    {stack}
                    onEdit={() => InteractedStackStore.displaySaveModal(stack.id)}
                    onDelete={() => InteractedStackStore.displayDeleteModal(stack.id ?? 0)}
                />
            {/each}
        {:else}
            <article class="flex flex-row justify-between py-2 px-4 bg-base-25 w-2xl mx-auto rounded-md relative">
                {#if id > 0}
                    <p>Aucune pile n'est attachée à cette étagère.</p>
                {:else}
                    <p>Aucune pile flottante n'existe.</p>
                {/if}
            </article>
        {/if}
    </div>
    <button
        class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
        onclick={() => InteractedStackStore.displaySaveModal()}
    >Créer une nouvelle pile</button>
</main>
{#if $showSaveShelfModalStore}
    <SaveShelfModal onSave={refresh} />
{/if}
{#if $showDeleteShelfModalStore}
    <DeleteShelfModal onDelete={onDelete} />
{/if}
{#if $showSaveStackModalStore}
    <SaveStackModal onSave={refresh} />
{/if}
{#if $showDeleteStackModalStore}
    <DeleteStackModal onDelete={refresh} />
{/if}
