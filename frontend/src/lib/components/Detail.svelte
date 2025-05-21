<script lang="ts">
	export let id: number;
	export let type: 'movie' | 'tv' = 'movie';

	let detail: any = null;
	let loading = true;
	let error: string | null = null;

	$: if (id) loadDetail();

	async function loadDetail() {
		loading = true;
		error = null;
		try {
			const res = await fetch(`/api/detail?id=${id}&type=${type}`);
			if (!res.ok) throw new Error(await res.text());
			detail = await res.json();
		} catch (err: any) {
			error = err.message;
		} finally {
			loading = false;
		}
	}
</script>

{#if loading}
	<p class="text-gray-500 italic">Načítám...</p>
{:else if error}
	<p class="text-red-600">Chyba: {error}</p>
{:else if detail}
	<div class="mx-auto mt-8 max-w-3xl rounded-xl bg-white p-4 shadow">
		<h1 class="mb-2 text-2xl font-bold">{detail.title}</h1>
		<p class="mb-4 text-sm text-gray-500">Vydáno: {detail.release_date}</p>
		{#if detail.poster_url}
			<img src={detail.poster_url} alt={detail.title} class="mb-4 w-64 rounded-xl" />
		{/if}
		<p class="mb-2">{detail.overview}</p>
		<p class="text-sm text-gray-700">Žánry: {detail.genres.join(', ')}</p>
		<p class="text-sm text-yellow-600">Hodnocení: {detail.rating.toFixed(1)}/10</p>
	</div>
{/if}
