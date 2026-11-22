import type {Algorithm} from "$lib/types/Algorithm";

export type Stack = {
    id: number | null,
    shelf_id: number | null,
    name: string | null,
    algorithm: Algorithm | null,
    created_at: string | null,
    modified_at: string | null,
}