<script lang="ts">
    import type {CardTemplate} from "$lib/types/CardTemplate";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {X} from "@lucide/svelte";
    import {
        toggleDeleteCardTemplateModal
    } from "$lib/stores/card_templates/showDeleteCardTemplateModalStore";
    import {InteractedCardTemplateStore} from "$lib/stores/card_templates/InteractedCardTemplateStore";
    import {get} from "svelte/store";

    interface Props {
        onDelete: () => Promise<void>;
    }

    let {onDelete}: Props = $props();

    let id: number | null = get(InteractedCardTemplateStore.idStore);
    let card_template: CardTemplate = $state({id: null, note_template_id: null, name: null, front_markup: null, back_markup: null, created_at: null, modified_at: null});

    onMount(async () => {
        if (id !== null) card_template = await invoke("read_one_card_template", {id});
    });

    function close(): void {
        toggleDeleteCardTemplateModal(false);
    }

    async function trash(): Promise<void> {
        await invoke("delete_card_template", {id: id});
        toggleDeleteCardTemplateModal(false);
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
            >Supprimer « {card_template.name} »</p>
            <button
                class="absolute top-1 right-0 hover:text-red-500"
                onclick={trash}
            ><X size="20" /></button>
        </div>
        <p>En supprimant un modèle de carte, vous supprimerez les cartes l'exploitant.</p>
        <button
            class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            type="button"
            onclick={trash}
        >Supprimer</button>
    </div>
</div>