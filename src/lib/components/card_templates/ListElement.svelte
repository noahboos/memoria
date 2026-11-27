<script lang="ts">
    import type {CardTemplate} from "$lib/types/CardTemplate";

    interface Props {
        card_template: CardTemplate,
        onDelete: (id: number) => void,
    }

    let {card_template, onDelete}: Props = $props();
</script>

<article class="flex flex-row justify-between bg-base-25 w-2xl mx-auto rounded-md hover:bg-accent-500/35 relative">
    <a
        class="py-2 px-4 max-w-fit underline after:content-[''] after:absolute after:inset-0"
        href="{ "/card_templates/" + card_template.id }"
    >{card_template.name}</a>
    <div class="mr-2">
        <a
            class="w-fit underline my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1"
            href={"/card_templates/save/" + card_template.id + "?note_template_id=" + card_template.note_template_id }
        >Modifier</a>
        <button
            class="my-2 px-2 hover:bg-primary-200 hover:text-base-00 rounded-md relative z-1 cursor-pointer"
            onclick={() => card_template.id !== null && onDelete(card_template.id)}
        >Supprimer</button>
    </div>
</article>