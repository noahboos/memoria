<script lang="ts">
    import type {PageProps} from "../../../../../.svelte-kit/types/src/routes/card_templates/save/[id]/$types";
    import { page } from "$app/state";
    import type {CardTemplate} from "$lib/types/CardTemplate";
    import type {NoteTemplate} from "$lib/types/NoteTemplate";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {goto} from "$app/navigation";
    import {CardSanitizerService} from "$lib/services/CardSanitizerService";

    let {params}: PageProps = $props();
    let id: number = Number(params.id);
    let note_template_id: number = $derived(Number(page.url.searchParams.get("note_template_id")));

    let note_template: NoteTemplate = $state<NoteTemplate>({id: null, name: null, description: null, data: null, created_at: null, modified_at: null });
    let card_template: CardTemplate = $state<CardTemplate>({id: null, note_template_id, name: "", front_markup: "", back_markup: "", created_at: null, modified_at: null,});

    onMount(async () => {
        note_template = await invoke("read_one_note_template", {id: note_template_id});

        if (id > 0) {
            card_template = await invoke("read_one_card_template", {id});

            await getCurrentWindow().setTitle("Mettre à jour le modèle de carte « " + card_template.name + " » | Memoria");
        } await getCurrentWindow().setTitle("Créer un modèle de carte | Memoria")
    });

    async function save(): Promise<void> {
        if (typeof card_template.front_markup !== "string" || typeof card_template.back_markup !== "string") return;
        card_template.front_markup = CardSanitizerService.sanitize(card_template.front_markup);
        card_template.back_markup = CardSanitizerService.sanitize(card_template.back_markup);

        await invoke("save_card_template", {cardTemplate: card_template});

        await goto("/note_templates/" + card_template.note_template_id);
    }
</script>

<main class="p-4 space-y-4">
    <div class="space-y-1">
        <p class="font-serif text-2xl text-center">
            {#if id > 0}
                Mettre à jour le modèle de « {card_template.name} »
            {:else}
                Créer un nouveau modèle de carte
            {/if}
        </p>
        <p class="font-serif text-lg text-center">En lien avec le modèle de note « {note_template.name} »</p>
    </div>
    <form class="space-y-4" onsubmit={save}>
        <div class="flex flex-col gap-2">
            <label
                class="font-bold"
                for="name"
            >Nom</label>
            <input
                class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 "
                id="name"
                type="text"
                bind:value={card_template.name}
                placeholder="Exemple : « Modèle de carte - Du français au polonais »"
                required
            />
        </div>
        <div class="flex flex-col gap-2">
            <label
                class="font-bold"
                for="front_markup"
            >Balisage avant</label>
            <textarea
                class="font-mono bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
                id="front_markup"
                bind:value={card_template.front_markup}
                placeholder=""
                required
            ></textarea>
        </div>
        <div class="flex flex-col gap-2">
            <label
                class="font-bold"
                for="back_markup"
            >Balisage arrière</label>
            <textarea
                class="font-mono bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
                id="back_markup"
                bind:value={card_template.back_markup}
                placeholder=""
                required
            ></textarea>
        </div>
        <button
            class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            type="submit"
        >Enregistrer</button>
    </form>
</main>