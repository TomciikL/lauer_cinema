<script lang="ts">
	import { page } from '$app/stores';
	import '../app.css';

	let { children } = $props();
</script>

<nav class="border-b border-gray-300 bg-gray-100">
	<div class="flex items-center justify-center px-4">
		<a href="/" class="font-bold text-blue-600 hover:text-blue-800">LC</a>
		<div
			class="container mx-auto flex justify-center gap-6 px-4 py-3 text-base font-medium text-gray-700"
		>
			<a
				href="/movies"
				class="transition-colors hover:text-black {$page.url.pathname.startsWith('/movies')
					? 'font-semibold text-blue-600'
					: ''}"
			>
				Filmy
			</a>
			<a
				href="/tv"
				class="transition-colors hover:text-black {$page.url.pathname.startsWith('/tv')
					? 'font-semibold text-blue-600'
					: ''}"
			>
				Seriály
			</a>
		</div>
	</div>
</nav>

<div class="container mx-auto">
	{@render children()}
</div>
