<script lang="ts">
import { Button, Checkbox, Input, Select, Toggle } from 'flowbite-svelte';
import { Dialect, type LintConfig } from 'harper.js';
import logo from '/logo.png';
import ProtocolClient from '../ProtocolClient';

let lintConfig: LintConfig = $state({});
let lintDescriptions: Record<string, string> = $state({});
let searchQuery = $state('');
let searchQueryLower = $derived(searchQuery.toLowerCase());
let dialect = $state(Dialect.American);
let defaultEnabled = $state(false);
let userDict = $state('');

$effect(() => {
	ProtocolClient.setLintConfig(lintConfig);
});

$effect(() => {
	ProtocolClient.setDialect(dialect);
});

$effect(() => {
	ProtocolClient.setDefaultEnabled(defaultEnabled);
});

$effect(() => {
	console.log('hit');
	ProtocolClient.setUserDictionary(stringToDict(userDict));
});

ProtocolClient.getLintConfig().then((l) => {
	lintConfig = l;
});

ProtocolClient.getLintDescriptions().then((d) => {
	lintDescriptions = d;
});

ProtocolClient.getDialect().then((d) => {
	dialect = d;
});

ProtocolClient.getDefaultEnabled().then((d) => {
	defaultEnabled = d;
});

ProtocolClient.getUserDictionary().then((d) => {
	userDict = dictToString(d.toSorted());
});

function configValueToString(value: boolean | undefined): string {
	switch (value) {
		case true:
			return 'enable';
		case false:
			return 'disable';
		case undefined:
		case null:
			return 'default';
	}
}

function configStringToValue(str: string): boolean | undefined | null {
	switch (str) {
		case 'enable':
			return true;
		case 'disable':
			return false;
		case 'default':
			return null;
	}

	throw 'Fell through case';
}

/** Converts the content of a text area to viable dictionary values. */
export function stringToDict(s: string): string[] {
	return s
		.split('\n')
		.map((s) => s.trim())
		.filter((v) => v.length > 0);
}

/** Converts the content of a text area to viable dictionary values. */
export function dictToString(values: string[]): string {
	return values.map((v) => v.trim()).join('\n');
}
</script>

<!-- centered wrapper with side gutters -->
<div class="mx-auto max-w-screen-md px-4">
  <header class="flex items-center gap-2 px-3 py-2 bg-gray-50/60 border-b border-gray-200 rounded-t-lg">
    <img src={logo} alt="Harper logo" class="h-6 w-auto" />
    <span class="font-semibold text-sm">Harper</span>
  </header>

  <main class="p-6 space-y-10 text-sm text-gray-800 border border-gray-200 rounded-b-lg shadow-sm">
    <!-- ── GENERAL ───────────────────────────── -->
    <section class="space-y-6">
      <h3 class="pb-1 border-b border-gray-200 text-xs font-semibold uppercase tracking-wider text-gray-500">General</h3>

      <div class="space-y-5">
        <div class="flex items-center justify-between">
          <span class="font-medium">English Dialect</span>
          <Select size="sm" color="primary" class="w-44" bind:value={dialect}>
            <option value={Dialect.American}>🇺🇸 American</option>
            <option value={Dialect.British}>🇬🇧 British</option>
            <option value={Dialect.Australian}>🇦🇺 Australian</option>
            <option value={Dialect.Canadian}>🇨🇦 Canadian</option>
          </Select>
        </div>
      </div>

      <div class="space-y-5">
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="font-medium">Enable on New Sites by Default</span>
            <span class="font-light">Can make some apps behave abnormally.</span>
          </div>
          <input type="checkbox" bind:checked={defaultEnabled}/>
        </div>
      </div>

      <div class="space-y-5">
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="font-medium">User Dictionary</span>
            <span class="font-light">Each word should be on its own line.</span>
          </div>
          <textarea bind:value={userDict} />
        </div>
      </div>

    </section>

    <!-- ── RULES ─────────────────────────────── -->
    <section class="space-y-4">
      <div class="flex items-center justify-between gap-4">
        <h3 class="text-xs font-semibold uppercase tracking-wider text-gray-500">Rules</h3>
        <Input bind:value={searchQuery} placeholder="Search for a rule…" size="sm" class="w-60" />
      </div>

      {#each Object.entries(lintConfig).filter(([key]) => lintDescriptions[key].toLowerCase().includes(searchQueryLower) || key.toLowerCase().includes(searchQueryLower)) as [key, value]}
        <div class="space-y-4 max-h-80 overflow-y-auto pr-1">
          <!-- rule card sample -->
          <div class="rounded-lg border border-gray-200 p-3 shadow-sm">
            <div class="flex items-start justify-between gap-4">
              <div class="space-y-0.5">
                <p class="font-medium">{key}</p>
                <p class="text-xs text-gray-600">{@html lintDescriptions[key]}</p>
              </div>
              <Select
                size="sm"
                value={configValueToString(value)}
                on:change={(e) => {
                  lintConfig[key] = configStringToValue(e.target.value);
                }}
                class="max-w-[10rem]"
              >
                <option value="default">⚙️ Default</option>
                <option value="enable">✅ On</option>
                <option value="disable">🚫 Off</option>
              </Select>
            </div>
          </div>
        </div>
      {/each}

    </section>
  </main>
</div>
