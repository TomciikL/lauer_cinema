import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		id: Number(params.id),
		type: 'tv' as const
	};
};
