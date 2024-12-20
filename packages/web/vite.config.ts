import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import { defineConfig } from 'vite';
import { sveltepress } from '@sveltepress/vite';
import { defaultTheme } from '@sveltepress/theme-default';

export default defineConfig({
	server: {
		port: 3000,
		fs: {
			allow: ['../harper.js/dist']
		}
	},
	plugins: [
		sveltepress({
			siteConfig: {
				title: 'Harper',
				description: 'A Grammar Checker from Automattic'
			},
			theme: defaultTheme({
				logo: '/circle-logo.png',
				github: 'https://github.com/automattic/harper',
				themeColor: {
					primary: '#818eae',
					dark: '#355280',
					gradient: {
						start: '#355280',
						end: '#818eae'
					}
				},
				navbar: [{ title: 'Documentation', to: '/docs/about' }],
				sidebar: {
					'/docs/': [
						{
							title: 'About',
							to: '/docs/about'
						},
						{
							title: 'Integrations',
							to: '/docs/integrations'
						}
					]
				},
				highlighter: {
					languages: ['svelte', 'sh', 'js', 'html', 'ts', 'md', 'css', 'scss', 'toml', 'rust']
				}
			})
		}),
		wasm(),
		topLevelAwait()
	]
});
