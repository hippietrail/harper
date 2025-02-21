<script module lang="ts">
	import { LocalLinter } from 'harper.js';

	export const prerender = true;
	export const frontmatter = {
		title: 'Rules'
	};

	let info: Record<string, string> = $state({});

	let linter = new LocalLinter();
	linter.getLintDescriptions().then(async (v) => {
		info = v;
		console.log(v);
	});
</script>

<p>This page is an incomplete list of the various grammatical rules Harper checks for.</p>

{#each Object.entries(info) as [name, description]}
	<h2>{name}</h2>
	<p>{description}</p>
{/each}
