<script module lang="ts">
	import { LocalLinter, type LintConfig } from 'harper.js';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';

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

<Table>
	<TableHead>
		<TableHeadCell>Name</TableHeadCell>
		<TableHeadCell>Enabled by Default</TableHeadCell>
		<TableHeadCell>Description</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each Object.entries(descriptions) as [name, description]}
			<TableBodyRow>
				<TableBodyCell>{name}</TableBodyCell>
				<TableBodyCell>{default_config[name] ? 'Enabled' : 'Disabled'}</TableBodyCell>
				<TableBodyCell>{description}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
