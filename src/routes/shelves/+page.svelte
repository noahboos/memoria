<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import type {Shelf} from "$lib/types/Shelf";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import ListElement from "$lib/components/shelves/ListElement.svelte";

    let shelves: Shelf[] | null = $state(null);

    onMount(async () => {
        await getCurrentWindow().setTitle("Liste des étagères | Memoria")

        shelves = await invoke('read_all_shelves');
        console.log(shelves);
    });
</script>

<main class="w-full p-4 space-y-4">
    <h1 class="font-serif text-2xl text-center">Étagères</h1>
    <div class="space-y-2">
        {#each shelves as shelf}
            <ListElement {shelf} />
        {/each}
    </div>
    <button
        class="block w-fit mx-auto bg-base-30 px-2 py-1 rounded-md hover:bg-primary-200 hover:text-base-00"
    >Créer une nouvelle étagère</button>
</main>
