type FetchOptions = RequestInit;
type FetchEndpoint = string;
type FetchFetcher = typeof fetch;

export const fetchData = async <T>(
	fetcher: FetchFetcher,
	endpoint: FetchEndpoint,
	options: FetchOptions
): Promise<T> => {
	const response = await fetcher(endpoint, options);

	if (!response.ok) {
		throw new Error('Failed to fetch data');
	}

	return response.json();
};

export const fetchApi = async <T>(
	fetcher: FetchFetcher,
	endpoint: FetchEndpoint,
	options: FetchOptions
): Promise<T> => {
	const session = { accessToken: 'hoal' };

	const headers = {
		'Content-Type': 'application/json',
		...(session?.accessToken ? { Authorization: `Bearer ${session.accessToken}` } : {}),
		...options.headers
	};

	return fetchData(fetcher, endpoint, {
		...options,
		headers
	});
};
