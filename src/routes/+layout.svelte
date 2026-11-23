<script lang="ts">
    import "$lib/styles/style.css"
    import type {Snippet} from "svelte";
    import Toolbar from "$lib/components/toolbar/Toolbar.svelte";

    let { children }: { children: Snippet } = $props();

    $effect(() => {
        document.body.setAttribute("data-theme", "default");
        document.body.setAttribute("data-theme-variant", "dark");
    });
</script>

<div class="text-base text-base-100 bg-base-00 w-full h-screen">
    <Toolbar />
    {@render children()}
</div>