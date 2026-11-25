<script lang="ts">
    import type {NoteTemplate} from "$lib/types/NoteTemplate";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import ListElement from "$lib/components/note_templates/ListElement.svelte";
    import {showDeleteNoteTemplateModalStore} from "$lib/stores/note_templates/showDeleteNoteTemplateModalStore";
    import DeleteNoteTemplateModal from "$lib/components/note_templates/DeleteNoteTemplateModal.svelte";
    import {InteractedNoteTemplateStore} from "$lib/stores/note_templates/InteractedNoteTemplateStore";

    let note_templates: NoteTemplate[] = $state([]);

    onMount(async () => {
        await getCurrentWindow().setTitle("Liste des modèles de note | Memoria");

        note_templates = await invoke("read_all_note_templates");
    });

    async function refresh(): Promise<void> {
        note_templates = await invoke("read_all_note_templates");
    }
</script>

<main class="p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">Modèles de note</h1>
    <div class="space-y-2">
        {#if note_templates.length > 0}
            {#each note_templates as note_template}
                <ListElement
                    {note_template}
                    onDelete={() => InteractedNoteTemplateStore.displayDeleteModal(note_template.id ?? 0)}
                />
            {/each}
        {:else}
            <article class="flex flex-row justify-between py-2 px-4 bg-base-25 w-2xl mx-auto rounded-md relative">
                <p>Aucun modèle de note n'existe.</p>
            </article>
        {/if}
    </div>
    <a
        class="block w-fit mx-auto bg-base-30 underline px-2 py-1 rounded-md hover:bg-base-25"
        href="/note_templates/save/0"
    >Créer un nouveau modèle de note</a>
</main>

{#if $showDeleteNoteTemplateModalStore}
    <DeleteNoteTemplateModal onDelete={refresh} />
{/if}