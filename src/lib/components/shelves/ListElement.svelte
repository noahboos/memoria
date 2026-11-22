<script lang="ts">
    import type {Shelf} from "$lib/types/Shelf";

    interface Props {
        shelf: Shelf;
        onEdit: (id: number) => void;
    }

    let { shelf, onEdit }: Props = $props();
</script>

<article class="flex flex-row justify-between bg-base-25 w-2xl mx-auto rounded-md hover:bg-accent-500/35 relative">
    <a
        class="py-2 px-4 max-w-fit underline after:content-[''] after:absolute after:inset-0"
        href="{ "/shelf/" + shelf.id }"
    >{shelf.name}</a>
    <div class="mr-2">
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
            onclick={() => shelf.id !== null && onEdit(shelf.id)}
        >Modifier</button>
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
        >Supprimer</button>
    </div>
</article>