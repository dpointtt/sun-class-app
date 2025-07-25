import { create_assignment } from '$lib/api/classrooms';
import { load_class_info } from '$lib/api/classrooms';
import { get_user_role } from '$lib/api/user';
import type { ClassData } from '$lib/types';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const response: ClassData = await fetch(load_class_info(params.class_id))
		.then((response) => response.json())
		.catch(console.error);

	const role = await fetch(get_user_role(params.class_id))
		.then((response) => response.json())
		.catch(console.error);
	return {
		class: response,
		is_teacher: role.is_teacher
	};
};

export const actions = {
	createassignment: async ({ params, request, fetch }) => {
		const data = await request.formData();
		const title = data.get('title') as string;
		const description = data.get('description') as string;
		const due_date = data.get('due_date') as string;
		const max_points = data.get('max_points') as string;

		if (title === '') {
			return { success: false, error: 'Title is required.' };
		}

		if (due_date === '') {
			return { success: false, error: 'Due date is required.' };
		}

		//console.log(`Creating assignment for class ${params.class_id} with title "${title}", description "${description}", due date "${due_date}", and max points ${max_points}`);
		const response = await fetch(create_assignment(params.class_id, title, description, due_date, +max_points));

		if (!response.ok) {
			return { success: false };
		}
		return { success: true };
	}
} satisfies Actions;