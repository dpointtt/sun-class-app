import { list_submissions } from "$lib/api/classrooms";
import { teacher_state } from "$lib/stores/state.svelte";
import type { SubmissionListItem } from "$lib/types";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    // if (!teacher_state) {
    //     throw redirect(303, "/class/" + params.class_id);
    // }
    const response: SubmissionListItem[] = await fetch(list_submissions(params.class_id))
        .then((response) => response.json())
        .catch(console.error);
    return {
        submissions: response,
        class_id: params.class_id
    };
};