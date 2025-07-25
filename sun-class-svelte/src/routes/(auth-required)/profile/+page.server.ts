import type { Actions, PageServerLoad } from "./$types";
import { edit_profile_request, profile_request } from "$lib/api/user";
import type { UserProfileResponse } from "$lib/api/types";
import { invalidateAll } from "$app/navigation";

export const load: PageServerLoad = async ({ fetch }) => {
  const response: UserProfileResponse = await fetch(profile_request).then((response) => response.json()).catch(console.error);
  return {
    user: response
  };
};

export const actions = {
	edit_profile: async ({ request, fetch }) => {
		const data = await request.formData();
		const name = data.get('name') as string;

    if (name == "") {
      // todo
    }

    //console.log(edit_profile_request(name, "").headers.get('content-type'));
		const response = await fetch(edit_profile_request(name, "")).catch(console.error);

		return { success: true };
	}
} satisfies Actions;
