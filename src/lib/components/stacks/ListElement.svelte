<script lang="ts">
    import type {Stack} from "$lib/types/Stack";

    interface Props {
        stack: Stack;
        onEdit: (id: number) => void;
        onDelete: (id: number) => void;
    }

    let { stack, onEdit, onDelete }: Props = $props();
</script>

<article class="flex flex-row justify-between bg-base-25 w-2xl mx-auto rounded-md hover:bg-accent-500/35 relative">
    <a
        class="py-2 px-4 max-w-fit underline after:content-[''] after:absolute after:inset-0"
        href="{ "/stacks/" + stack.id }"
    >{stack.name}</a>
    <div class="mr-2">
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
            onclick={() => stack.id !== null && onEdit(stack.id)}
        >Modifier</button>
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
            onclick={() => stack.id !== null && onDelete(stack.id)}
        >Supprimer</button>
    </div>
</article>