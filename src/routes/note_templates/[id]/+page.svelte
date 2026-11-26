<script lang="ts">
    import type {PageProps} from "../../../../.svelte-kit/types/src/routes/note_templates/[id]/$types";
    import {onMount} from "svelte";
    import type {NoteTemplate} from "$lib/types/NoteTemplate";
    import {invoke} from "@tauri-apps/api/core";
    import type {CardTemplate} from "$lib/types/CardTemplate";
    import DeleteNoteTemplateModal from "$lib/components/note_templates/DeleteNoteTemplateModal.svelte";
    import {showDeleteNoteTemplateModalStore} from "$lib/stores/note_templates/showDeleteNoteTemplateModalStore";
    import {goto} from "$app/navigation";
    import {InteractedNoteTemplateStore} from "$lib/stores/note_templates/InteractedNoteTemplateStore";

    let {params}: PageProps = $props();

    let id: number = Number(params.id);
    let note_template: NoteTemplate = $state<NoteTemplate>({id: null, name: null, description: null, data: null, created_at: null, modified_at: null });
    let card_templates: CardTemplate[] = $state<CardTemplate[]>([]);

    onMount(async () => {
        note_template = await invoke("read_one_note_template", {id});
        card_templates = await invoke("read_all_card_templates_by_note_template_id", { noteTemplateId: id});
    });

    function onDelete(): void {
        goto("/note_templates/");
    }
</script>

<main class="p-4 space-y-4">
    <p class="font-serif text-2xl text-center">{note_template.name}</p>
    <div class="flex flex-row gap-2 justify-center">
        <a
            class="block w-fit bg-base-30 px-2 py-1 underline rounded-md hover:bg-primary-200 hover:text-base-00"
            href={"/note_templates/save/" + id}
        >Modifier</a>
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={() => InteractedNoteTemplateStore.displayDeleteModal(id)}
        >Supprimer</button>
    </div>
    <a
        class="block w-fit mx-auto bg-base-30 px-2 py-1 underline rounded-md hover:bg-primary-200 hover:text-base-00"
        href={"/card_templates/save/0?note_template_id=" + id}
    >Créer un nouveau modèle de carte</a>
</main>

{#if $showDeleteNoteTemplateModalStore}
    <DeleteNoteTemplateModal onDelete={async () => onDelete()} />
{/if}