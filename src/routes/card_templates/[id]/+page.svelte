<script lang="ts">
    import type {PageProps} from "../../../../.svelte-kit/types/src/routes/card_templates/[id]/$types";
    import type {CardTemplate} from "$lib/types/CardTemplate";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import DeleteCardTemplateModal from "$lib/components/card_templates/DeleteCardTemplateModal.svelte";
    import {showDeleteCardTemplateModalStore} from "$lib/stores/card_templates/showDeleteCardTemplateModalStore";
    import {goto} from "$app/navigation";
    import {InteractedCardTemplateStore} from "$lib/stores/card_templates/InteractedCardTemplateStore";

    let {params}: PageProps = $props();

    let id: number = Number(params.id);
    let card_template: CardTemplate = $state<CardTemplate>({id: null, note_template_id: null, name: "", front_markup: "", back_markup: "", created_at: null, modified_at: null,});

    onMount(async () => {
        card_template = await invoke("read_one_card_template", {id});
    });

    function onDelete() {
        goto("/note_templates/" + card_template.note_template_id);
    }
</script>

<main class="p-4 space-y-4">
    <p class="font-serif text-2xl text-center">{card_template.name}</p>
    <div class="flex flex-row gap-2 justify-center">
        <a
            class="block w-fit bg-base-30 px-2 py-1 underline rounded-md hover:bg-primary-200 hover:text-base-00"
            href={"/card_templates/save/" + id + "?note_template_id=" + card_template.note_template_id}
        >Modifier</a>
        <button
            class="block w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            onclick={() => InteractedCardTemplateStore.displayDeleteModal(id)}
        >Supprimer</button>
    </div>
</main>
{#if $showDeleteCardTemplateModalStore}
    <DeleteCardTemplateModal onDelete={async () => onDelete()} />
{/if}