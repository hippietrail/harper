<script module lang="ts">
	import { LocalLinter, type LintConfig } from 'harper.js';

	export const prerender = true;
	export const frontmatter = {
		title: 'Rules'
	};

	let descriptions: Record<string, string> = $state({});
	let default_config: LintConfig = $state({});

	let linter = new LocalLinter();
	linter.getLintDescriptions().then(async (v) => {
		descriptions = v;
	});
	linter.getDefaultLintConfig().then(async (v) => {
		default_config = v;
	});
</script>

<p>This page is an incomplete list of the various grammatical rules Harper checks for.</p>

{#each Object.entries(descriptions) as [name, description]}
	<h2>{name}</h2>
	<p>{description}</p>
	<p>This rule is {default_config[name] ? 'enabled' : 'disabled'} by default.</p>
{/each}
