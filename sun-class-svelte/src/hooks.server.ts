import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
    //request.headers.set('content-type', "application/json;charset=UTF-8");
	request.headers.set('cookie', event.request.headers.get('cookie') ?? '');
	return fetch(request);
};