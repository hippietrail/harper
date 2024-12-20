<script lang="ts">
	import { page } from '$app/stores';

	export let pages: Record<string, string> = {};

	function leaf(url: string): string {
		return url.split('/').pop() || '/';
	}

	$: currentPage = leaf($page.url.pathname);
</script>

<div class="w-48 m-2 p-2 border-black dark:border-white border-t-2 border-l-2 h-full">
	{#each Object.entries(pages) as [href, title]}
		<a {href}>
			<div
				class={`transition-all hover:translate-x-2 ${currentPage == leaf(href) ? 'translate-x-1 font-bold' : ''}`}
			>
				{title}
			</div></a
		>
	{/each}
</div>
