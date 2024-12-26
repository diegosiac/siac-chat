import { SvelteKitAuth } from '@auth/sveltekit';
import Credentials from '@auth/sveltekit/providers/credentials';
import { env } from '$env/dynamic/private';
import { refreshTokenSession } from '@/features/auth/helpers';

const BACKEND_ACCESS_TOKEN_LIFETIME = 60 * 60 * 2 - 60; // 120 minutes
const BACKEND_REFRESH_TOKEN_LIFETIME = 60 * 60 * 24 - 60; // 1 day

const getCurrentEpochTime = () => {
	return Math.floor(new Date().getTime() / 1000);
};

const getNewRef = () => {
	return getCurrentEpochTime() + BACKEND_ACCESS_TOKEN_LIFETIME;
};

export const { handle, signIn, signOut } = SvelteKitAuth({
	secret: env.AUTH_SECRET,
	pages: {
		signIn: '/',
		signOut: '/'
	},
	session: {
		strategy: 'jwt',
		maxAge: BACKEND_REFRESH_TOKEN_LIFETIME
	},
	callbacks: {
		async jwt({ token, user, account }) {
			if (user && account) {
				const { access, refresh, ...userData } = user;

				token['user'] = userData;
				token['access'] = access;
				token['refresh'] = refresh;
				token['ref'] = getNewRef();

				return token;
			}

			if (getCurrentEpochTime() > (token['ref'] as number)) {
				const response = await refreshTokenSession(token['refresh'] as string);

				token['access'] = response.access;
				token['refresh'] = response.refresh;
				token['ref'] = getNewRef();
			}

			return token;
		}
	},
	providers: [
		Credentials({
			authorize: async (credentials) => {
				const user = true;

				if (!user) {
					throw new Error('User not found.');
				}

				return {
					id: 'string',
					name: 'string',
					email: 'string',
					image: 'string'
				};
			}
		})
	]
});
