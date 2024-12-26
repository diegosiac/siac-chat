import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export const load = async (event) => {
	const session = await event.locals.auth();

	if (!session) {
		redirect(307, '/');
	}

	return {
		session
	};
};
