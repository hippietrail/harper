<script>
	import { LocalLinter } from 'harper.js';
	import { Button } from 'flowbite-svelte';

	let linter = new LocalLinter();

	let head = `lspconfig.harper_ls.setup {
  settings = {
    ["harper-ls"] = {
      linters = {
`;

	let tail = `      }
    }
  },
}`;

	async function generateConfig() {
		await linter.setLintConfigToDefault();
		let config = await linter.getLintConfig();

		let rows = Object.entries(config)
			.map(([key, value]) => `\t${key} = ${value},`)
			.reduce((prev, cur) => prev + '\n' + cur);

		return head + rows + tail;
	}

	async function copyConfig() {
		let defaultConfig = await generateConfig();
		navigator.clipboard.writeText(defaultConfig);
	}
</script>

<Button onclick={copyConfig}>Copy Default Config</Button>
