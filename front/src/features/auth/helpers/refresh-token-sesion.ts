import { env } from "$env/dynamic/private";
import { fetchData } from "@/lib";

export const refreshTokenSession = async (refreshToken: string) => {
		const response = await fetchData<{
					access: string;
					refresh: string;
				}>(fetch, `${env.BACKEND_URL}/auth/token/refresh`, {
					method: 'post',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ refresh: refreshToken })
				});
		
		return response;
}