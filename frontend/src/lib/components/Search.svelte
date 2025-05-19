<script lang="ts">
	import { writable } from 'svelte/store';

	let query = '';
	let results = writable([]);
	let loading = writable(false);
	let error = writable('');

	async function searchMovies() {
		if (!query.trim()) return;
		loading.set(true);
		error.set('');
		try {
			const res = await fetch(`/api/search?query=${encodeURIComponent(query)}`);
			if (!res.ok) throw new Error(`Chyba: ${res.status}`);
			const data = await res.json();
			results.set(data);
		} catch (err) {
			error.set(err instanceof Error ? err.message : 'Neznámá chyba');
			results.set([]);
		} finally {
			loading.set(false);
		}
	}

	function handleEnter(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			searchMovies();
		}
	}
</script>

<div class="mx-auto w-full max-w-2xl p-4">
	<div class="flex gap-2">
		<input
			class="w-full rounded-2xl border border-gray-300 px-4 py-2 text-base shadow-sm focus:border-blue-500 focus:ring focus:outline-none"
			placeholder="Hledat film..."
			bind:value={query}
			on:keydown={handleEnter}
		/>
		<button
			class="rounded-2xl bg-blue-600 px-4 py-2 text-white transition hover:bg-blue-700"
			on:click={searchMovies}
		>
			Hledat
		</button>
	</div>

	{#if $loading}
		<p class="mt-4 text-gray-600">Načítání…</p>
	{:else if $error}
		<p class="mt-4 text-red-600">{$error}</p>
	{:else if $results.length === 0 && query}
		<p class="mt-4 text-gray-500">Žádné výsledky</p>
	{:else}
		<ul class="mt-6 space-y-4">
			{#each $results as movie (movie.id)}
				<li class="rounded-xl border bg-white p-4 shadow">
					<div class="flex flex-row gap-4">
						<!-- Poster -->
						<img
							src={movie.poster_url}
							alt="Poster {movie.title}"
							class="h-32 w-20 rounded object-cover"
							loading="lazy"
						/>

						<!-- Textový obsah -->
						<div class="flex-1">
							<div class="mb-2 flex flex-col sm:flex-row sm:items-start sm:justify-between">
								<h2 class="text-lg font-semibold">
									{movie.title} ({new Date(movie.release_date).getFullYear()})
								</h2>
								<p class="text-sm text-gray-500 sm:text-right">
									{movie.genres.map((genre) => genre).join(', ')}
								</p>
							</div>
							<p class="text-sm text-gray-600">{movie.overview}</p>
						</div>
					</div>
				</li>
			{/each}
		</ul>
	{/if}
</div>
