<script lang="ts">
    import "$lib/styles/style.css"
    import type {Snippet} from "svelte";

    let { children }: { children: Snippet } = $props();
</script>

{@render children()}

<style>

</style>