<script lang="ts">
	import Detail from '$lib/components/Detail.svelte';
	export let data: { id: number; type: 'movie' };
</script>

<Detail id={data.id} type={data.type} />
