import './index.js';
import { startCase } from 'lodash-es';
import { type App, PluginSettingTab, Setting } from 'obsidian';
import type HarperPlugin from './index.js';
import type { Settings } from './index.js';

export class HarperSettingTab extends PluginSettingTab {
	private plugin: HarperPlugin;
	private settings: Settings;
	private descriptions: Record<string, string>;

	constructor(app: App, plugin: HarperPlugin) {
		super(app, plugin);
		this.plugin = plugin;

		this.updateDescriptions();
		this.updateSettings();
	}

	updateSettings() {
		this.plugin.getSettings().then((v) => (this.settings = v));
	}

	updateDescriptions() {
		this.plugin.getDescriptions().then((v) => (this.descriptions = v));
	}

	display() {
		const { containerEl } = this;
		containerEl.empty();

		console.log(this.settings.lintSettings);

		new Setting(containerEl).setName('Use Web Worker').addToggle((toggle) =>
			toggle.setValue(this.settings.useWebWorker).onChange(async (value) => {
				this.settings.useWebWorker = value;
				await this.plugin.initializeFromSettings(this.settings);
			}),
		);

		for (const setting of Object.keys(this.settings.lintSettings)) {
			const value = this.settings.lintSettings[setting];
			const description = this.descriptions[setting];

			new Setting(containerEl)
				.setName(startCase(setting))
				.setDesc(description)
				.addDropdown((dropdown) =>
					dropdown
						.addOption('default', 'Default')
						.addOption('enable', 'On')
						.addOption('disable', 'Off')
						.setValue(valueToString(value))
						.onChange(async (value) => {
							this.settings.lintSettings[setting] = stringToValue(value);
							await this.plugin.initializeFromSettings(this.settings);
						}),
				);
		}

		new Setting(containerEl).setName('The Danger Zone').addButton((button) => {
			button
				.setButtonText('Forget Ignored Suggestions')
				.onClick(() => {
					this.settings.ignoredLints = undefined;
					this.plugin.initializeFromSettings(this.settings);
				})
				.setWarning();
		});
	}
}

function valueToString(value: boolean | undefined): string {
	switch (value) {
		case true:
			return 'enable';
		case false:
			return 'disable';
		case null:
			return 'default';
	}

	throw 'Fell through case';
}

function stringToValue(str): boolean | undefined {
	switch (str) {
		case 'enable':
			return true;
		case 'disable':
			return false;
		case 'default':
			return undefined;
	}

	throw 'Fell through case';
}
