import '@auth/sveltekit';

declare module '@auth/sveltekit' {
	interface Session {
		access: string;
		refresh: string;
	}

	interface User extends UserApp {
		access: string;
		refresh: string;
	}
}
