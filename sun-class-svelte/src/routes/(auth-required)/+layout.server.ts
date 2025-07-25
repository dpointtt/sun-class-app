import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { get_user } from '$lib/api/user';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
  const authToken = cookies.get('auth_token');
  if (!authToken) {
    throw redirect(303, '/login');
  }
  const response = await fetch(get_user);
  if (response.status === 404 || response.status === 500) {
    cookies.delete('auth_token', { path: '/' });
    throw redirect(303, '/login');
  }
};