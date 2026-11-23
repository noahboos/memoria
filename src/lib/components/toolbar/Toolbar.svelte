<script lang="ts">
    import {
        isStackWithoutShelfIdExistingStore,
        refreshIsStackWithoutShelfIdExistingStore
    } from "$lib/stores/stacks/isStackWithoutShelfIdExistingStore";
    import {onMount} from "svelte";

    onMount(async () => {
        await refreshIsStackWithoutShelfIdExistingStore();
    });
</script>

<aside class="px-4 py-2 space-y-4 space-x-1">
    <a
        class="w-fit px-2 py-1 bg-base-30 text-sm underline rounded-md hover:bg-base-25 hover:cursor-pointer"
        href="/shelves/"
    >Étagères</a>
    {#if $isStackWithoutShelfIdExistingStore}
        <a
            class="w-fit px-2 py-1 bg-base-30 text-sm underline rounded-md hover:bg-base-25 hover:cursor-pointer"
            href="/shelves/0"
        >Piles flottantes</a>
    {/if}
</aside>