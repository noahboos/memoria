<script lang="ts">
    import type {PageProps} from "../../../../.svelte-kit/types/src/routes/stacks/[id]/$types";
    import type {Stack} from "$lib/types/Stack";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {showSaveStackModalStore, toggleSaveStackModal} from "$lib/hooks/stacks/showSaveStackModalStore";
    import {showDeleteStackModalStore, toggleDeleteStackModal} from "$lib/hooks/stacks/showDeleteStackModalStore";
    import DeleteStackModal from "$lib/components/stacks/DeleteStackModal.svelte";
    import SaveStackModal from "$lib/components/stacks/SaveStackModal.svelte";
    import {goto} from "$app/navigation";

    let {params}: PageProps = $props();

    const id: number = Number(params.id);
    let stack: Stack = $state({ id: null, shelf_id: null, name: null, algorithm: null, created_at: null, modified_at: null });

    onMount(async () => {
        await refresh();

        await getCurrentWindow().setTitle("Détail de la pile « " + stack.name + " » | Memoria");
    });

    function editStack(): void {
        toggleSaveStackModal(true);
    }

    function deleteStack(): void {
        toggleDeleteStackModal(true);
    }

    async function refresh(): Promise<void> {
        stack = await invoke("read_one_stack", {id});
    }

    async function onDelete(): Promise<void> {
        if (stack.shelf_id !== null && stack.shelf_id > 0) {
            await goto("/shelves/" + stack.shelf_id);
        } else {
            await goto("/shelves");
        }
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">{stack.name}</h1>
    <div class="flex flex-row gap-2 justify-center">
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={editStack}
        >Modifier</button>
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={deleteStack}
        >Supprimer</button>
    </div>
</main>
{#if $showSaveStackModalStore}
    <SaveStackModal id={id} shelf_id={stack.shelf_id} onSave={refresh} />
{/if}
{#if $showDeleteStackModalStore}
    <DeleteStackModal id={id} onDelete={onDelete} />
{/if}
