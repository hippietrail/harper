import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/kit/vite';
import { mdsvex, escapeSvelte } from 'mdsvex';
import hljs from 'highlight.js/lib/common';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', '.md'],
	preprocess: [
		vitePreprocess(),
		mdsvex({
			extensions: ['.md'],
			smartypants: true,
			highlight: {
				highlighter: async (code, language = 'text') => {
					const html = escapeSvelte(hljs.highlight(code, { language }).value);
					return `{@html \` <code class="p-4 rounded w-full block">${html}</code>\` }`;
				}
			}
		})
	],

	kit: {
		adapter: adapter({
			out: 'build'
		})
	}
};

export default config;
