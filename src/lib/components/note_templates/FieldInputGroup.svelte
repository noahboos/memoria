<script lang="ts">
    import {Minus} from "@lucide/svelte";
    import type {NoteTemplateField} from "$lib/types/NoteTemplateField";

    interface Props {
        fields: NoteTemplateField[],
        field: NoteTemplateField
    }

    let {fields, field}: Props = $props();

    const index = fields.indexOf(field);

    function remove(index: number): void {
        fields.splice(index, 1);
    }
</script>

<div class="flex flex-col gap-2 bg-base-20 p-4 rounded-md">
    <div class="flex flex-row justify-between">
        <p class="font-serif text-lg">
            {#if field.name === ""}
                {"Champ n°" + (index + 1)}
            {:else}
                {"Champ n°" + (index + 1) + " : " + field.name}
            {/if}
        </p>
        <button
            class="hover:text-red-500"
            type="button"
            onclick={() => remove(index)}
        ><Minus size="20" /></button>
    </div>
    <div class="flex flex-col gap-2">
        <label
            class="font-bold"
            for={"type-" + index}
        >Type</label>
        <select
            class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50"
            id={"type-" + index}
            bind:value={field.type}
        >
            <option value="text">Texte</option>
        </select>
    </div>
    <div class="flex flex-col gap-2">
        <label
            class="font-bold"
            for={"identifier" + index}
        >Identifiant</label>
        <input
            class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 "
            id={"identifier" + index}
            type="text"
            bind:value={field.identifier}
            placeholder="Exemple : « traduction_polonaise »"
            required
        />
    </div>
    <div class="flex flex-col gap-2">
        <label
            class="font-bold"
            for={"name" + index}
        >Nom</label>
        <input
            class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 "
            id={"name" + index}
            type="text"
            bind:value={field.name}
            placeholder="Exemple : « Traduction polonaise »"
            required
        />
    </div>
    <div class="flex flew-row gap-2">
        <label
            class="font-bold order-2"
            for={"required" + index}
        >Est obligatoire ?</label>
        <input
            class="bg-base-30 rounded-sm px-2 py-1 placeholder-accent-100/50 order-1"
            id={"required" + index}
            type="checkbox"
            bind:checked={field.required}
        />
    </div>
</div>