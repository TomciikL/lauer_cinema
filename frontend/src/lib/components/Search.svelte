<script lang="ts">
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';

	export let type: 'movie' | 'tv' = 'movie';

	const query = writable('');
	const results = writable([]);

	async function search() {
		const q = $query.trim();
		if (!q) return;

		const res = await fetch(`/api/search?query=${encodeURIComponent(q)}&media_type=${type}`);
		if (!res.ok) {
			console.error(await res.text());
			return;
		}
		results.set(await res.json());
	}

	function goToDetail(id: number) {
		goto(`/detail/${type}/${id}`);
	}
</script>

<div class="mx-auto max-w-2xl space-y-4 p-4">
	<div class="flex space-x-2">
		<input
			type="text"
			bind:value={$query}
			placeholder="Hledat..."
			class="w-full rounded-lg border px-4 py-2"
			on:keydown={(e) => e.key === 'Enter' && search()}
		/>
		<button on:click={search} class="rounded-lg bg-blue-600 px-4 py-2 text-white">Hledat</button>
	</div>

	{#if $results.length > 0}
		<ul class="space-y-4">
			{#each $results as item (item.id)}
				<li
					class="flex cursor-pointer space-x-4 rounded-lg border p-4 transition hover:bg-gray-50"
					on:click={() => goToDetail(item.id)}
				>
					{#if item.poster_url}
						<img src={item.poster_url} alt={item.title} class="h-36 w-24 rounded object-cover" />
					{/if}

					<div class="flex-1">
						<div
							class="mb-2 flex flex-col items-baseline sm:flex-row sm:items-baseline sm:justify-between"
						>
							<h2 class="text-lg font-semibold">
								{item.title} ({new Date(item.release_date).getFullYear()})
							</h2>
							<p class="text-sm text-gray-500 sm:text-right">
								{item.genres.join(', ')}
							</p>
						</div>
						<p class="text-sm text-gray-600">{item.overview}</p>
					</div>
				</li>
			{/each}
		</ul>
	{:else}
		<p class="text-center text-gray-500">Zadejte hledaný výraz.</p>
	{/if}
</div>
