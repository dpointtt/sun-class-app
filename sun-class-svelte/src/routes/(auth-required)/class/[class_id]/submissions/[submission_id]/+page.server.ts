import { cancel_grade, get_submission, grade_submission } from "$lib/api/classrooms";
import type { SubmissionResponse } from "$lib/types";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const response: SubmissionResponse = await fetch(get_submission(params.class_id, params.submission_id))
        .then((response) => response.json())
        .catch(console.error);
    return {
        submission_data: response,
        class_id: params.class_id
    };
};

export const actions = {
    savegrade: async ({ request, fetch, params }) => {
        const data = await request.formData();
        const grade = data.get('grade') as string;

        if (grade === null || grade === undefined) {
            return;
        }

        const response = await fetch(grade_submission(params.class_id, params.submission_id, +grade));

        if (!response.ok) {
            console.error("Failed to save the grade", response.statusText);
            return { success: false, error: "Failed to save the grade" };
        }
        return { success: true };
    },
    cancelgrade: async ({ fetch, params }) => {
        console.log("canceling grade, class id = " + params.class_id + ", submission_id = " + params.submission_id);
        const response = await fetch(cancel_grade(params.class_id, params.submission_id));

        if (!response.ok) {
            console.error("Failed to cancel the grade", response);
            return { success: false, error: "Failed to cancel the grade" };
        }
        return { success: true };
    }
} satisfies Actions;