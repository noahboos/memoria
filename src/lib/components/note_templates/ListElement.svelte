<script lang="ts">
    import type {NoteTemplate} from "$lib/types/NoteTemplate";

    interface Props {
        note_template: NoteTemplate,
        onDelete: (id: number) => void,
    }

    let {note_template, onDelete}: Props = $props();
</script>

<article class="flex flex-row justify-between bg-base-25 w-2xl mx-auto rounded-md hover:bg-accent-500/35 relative">
    <a
        class="py-2 px-4 max-w-fit underline after:content-[''] after:absolute after:inset-0"
        href="{ "/note_templates/" + note_template.id }"
    >{note_template.name}</a>
    <div class="mr-2">
        <a
            class="w-fit underline my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1"
            href={"/note_templates/save/" + note_template.id}
        >Modifier</a>
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
            onclick={() => note_template.id !== null && onDelete(note_template.id)}
        >Supprimer</button>
    </div>
</article>