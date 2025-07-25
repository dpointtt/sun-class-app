import { cancel_submission, delete_assignment_files, load_assignment_info, upload_assignment_files } from '$lib/api/classrooms';
import type { AssignmentData } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch }) => {
    const response: AssignmentData = await fetch(load_assignment_info(params.class_id, params.assignment_id))
            .then((response) => response.json())
            .catch(console.error);
    return response;
};

export const actions = {
    uploadassignmentfiles: async ({ request, params, fetch }) => {
        const data = await request.formData();
        //console.log(data.getAll("fileupload"));
        const response = await fetch(upload_assignment_files(params.class_id, params.assignment_id, data));
        if (!response.ok) {
            console.error("Failed to upload assignment files", response.statusText);
            return { success: false, error: "Failed to upload files" };
        }
        return { success: true };
    },
    deleteassignmentfile: async ({ request, params, fetch }) => {
        const data = await request.formData();
        const fileId = data.get('fileId') as string;
        console.log("Deleting file with ID:", fileId);
        const response = await fetch(delete_assignment_files(params.class_id, params.assignment_id, fileId));
        if (!response.ok) {
            console.error("Failed to delete assignment file", response.statusText);
            return { success: false, error: "Failed to delete file" };
        }
        return { success: true };
    },
    cancelsubmission: async ({ params, fetch }) => {
        console.log("Cancel submission for assignment:", params.assignment_id);
        const response = await fetch(cancel_submission(params.class_id, params.assignment_id));
        if (!response.ok) {
            console.error("Failed to delete assignment file", response.statusText);
            return { success: false, error: "Failed to delete file" };
        }
        return { success: true };
    }
};