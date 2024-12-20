<script lang="ts">
	import NavCol from '$lib/NavCol.svelte';
	import 'highlight.js/styles/github.min.css';

	let pages = {
		'./about': 'What Is Harper?',
		'./test2': 'Test2',
		'./integrations': 'Integrations'
	};
</script>

<div class="flex flex-row h-full">
	<NavCol {pages} />

	<div class="p-4">
		<slot />
	</div>
</div>
