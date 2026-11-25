<script lang="ts">
    import type {NoteTemplate} from "$lib/types/NoteTemplate";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {toggleDeleteNoteTemplateModal} from "$lib/stores/note_templates/showDeleteNoteTemplateModalStore";
    import {X} from "@lucide/svelte";
    import {InteractedNoteTemplateStore} from "$lib/stores/note_templates/InteractedNoteTemplateStore";
    import {get} from "svelte/store";

    interface Props {
        onDelete: () => Promise<void>,
    }

    let {onDelete}: Props = $props();

    let id: number | null = get(InteractedNoteTemplateStore.idStore);
    let note_template: NoteTemplate = $state({id: null, name: null, description: null, data: null, created_at: null, modified_at: null });

    onMount(async() => {
        if (id !== null) note_template = await invoke("read_one_note_template", {id});
    });

    function close(): void {
        toggleDeleteNoteTemplateModal(false);
    }

    async function trash(): Promise<void> {
        await invoke("delete_note_template", {id});
        toggleDeleteNoteTemplateModal(false);
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
            >Supprimer « {note_template.name} »</p>
            <button
                class="absolute top-1 right-0 hover:text-red-500"
                onclick={trash}
            ><X size="20" /></button>
        </div>
        <p>En supprimant un modèle de note, vous supprimerez ses modèles et les cartes l'exploitant.</p>
        <button
            class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            type="button"
            onclick={trash}
        >Supprimer</button>
    </div>
</div>