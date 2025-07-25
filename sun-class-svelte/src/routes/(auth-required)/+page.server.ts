import { get_classes } from "$lib/api/user";
import { create_class, join_class } from "$lib/api/classrooms";
import type { ClassList } from "$lib/types";
import type { PageServerLoad } from "./$types";
import type { Actions } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
  const response: ClassList = await fetch(get_classes)
    .then((response) => response.json())
    .catch(console.error);
  return response;
};

export const actions = {
  createclass: async ({ request, fetch }) => {
    const data = await request.formData();
    const title = data.get("title") as string;
    const description = data.get("description") as string;

    if (title === "") {
      return { success: false, error: "Title is required." };
    }

    const response = await fetch(create_class(title, description)).catch(
      console.error
    );

    if (!response?.ok) {
      return { success: false };
    }

    return { success: true };
  },
  joinclass: async ({ request, fetch }) => {
    const data = await request.formData();
    const joinCode = data.get("joinCode") as string;

    if (joinCode === "") {
      return { success: false, error: "joinCode is required." };
    }

    const response = await fetch(join_class(joinCode));

    if (!response.ok) {
      return { success: false };
    }

    return { success: true };
  },
} satisfies Actions;
