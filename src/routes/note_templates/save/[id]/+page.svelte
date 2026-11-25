<script lang="ts">
    import {onMount} from "svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import type {PageProps} from "../../../../../.svelte-kit/types/src/routes/note_templates/save/[id]/$types";
    import type {NoteTemplate} from "$lib/types/NoteTemplate";
    import type {CardTemplate} from "$lib/types/CardTemplate";
    import {invoke} from "@tauri-apps/api/core";
    import type {NoteTemplateField} from "$lib/types/NoteTemplateField";
    import FieldInputGroup from "$lib/components/note_templates/FieldInputGroup.svelte";
    import {goto} from "$app/navigation";

    let {params}: PageProps = $props();

    let id = Number(params.id);
    let note_template: NoteTemplate = $state<NoteTemplate>({id: null, name: null, description: null, data: null, created_at: null, modified_at: null });
    let fields: NoteTemplateField[] = $state([]);
    // let card_templates: CardTemplate[] = [];

    onMount(async() => {
        await getCurrentWindow().setTitle("Sauvegarder un modèle de note | Memoria");

        if (id > 0) {
            note_template = await invoke("read_one_note_template", {id});
            // card_templates = await invoke("read_all_card_templates_by_note_template_id", {id});
        }
    });

    function addField(): void {
        fields.push({
            type: "text",
            identifier: "",
            name: "",
            required: true
        })
    }

    async function save(): Promise<void> {
        note_template.data = JSON.stringify(fields);

        let _note_template = await invoke("save_note_template", { noteTemplate: note_template});

        await goto("/note_templates/");
    }
</script>

<main class="p-4 space-y-4">
    <p class="font-serif text-2xl text-center">
        {#if id > 0}
            Mettre à jour le modèle de note « {note_template.name} »
        {:else}
            Créer un nouveau modèle de note
        {/if}
    </p>
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
                bind:value={note_template.name}
                placeholder="Exemple : « Modèle de note - Vocabulaire »"
                required
            />
        </div>
        <div class="flex flex-col gap-2">
            <label
                class="font-bold"
                for="description"
            >Description</label>
            <input
                class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 "
                id="description"
                type="text"
                bind:value={note_template.description}
                placeholder="Exemple : « Un modèle de note utilisé pour générer des cartes mémoire afin d'apprendre du vocabulaire. »"
                required
            />
        </div>
        <div class="flex flex-row justify-between">
            <p class="font-serif text-xl">Champs</p>
            <button
                class="w-fit bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
                onclick={addField}
                type="button"
            >Ajouter un nouveau champ</button>
        </div>
        {#if fields.length === 0}
            <p
                class="flex flex-col gap-2 bg-base-20 py-2 px-4 rounded-md"
            >Aucun champ n'existe pour ce modèle de note.</p>
        {:else}
            {#each fields as field}
                <FieldInputGroup {fields} {field} />
            {/each}
        {/if}
        <button
            class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
            type="submit"
        >Enregistrer</button>
    </form>
</main>