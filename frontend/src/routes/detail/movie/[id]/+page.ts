import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		id: Number(params.id),
		type: 'movie' as const
	};
};
