import type { Extension } from 'vscode';

import { ConfigurationTarget, type Uri, commands, workspace } from 'vscode';

import {
	activateHarper,
	compareActualVsExpectedDiagnostics,
	createExpectedDiagnostics,
	createRange,
	getActualDiagnostics,
	openFile,
	sleep,
} from './helper';

describe('IntegrationDialect >', () => {
	let harper: Extension<void>;
	let markdownUri: Uri;

	beforeAll(async () => {
		harper = await activateHarper();
		// Open test file so diagnostics can occur
		markdownUri = await openFile('integrationBritish.md');
		// Wait for `harper-ls` to start
		await sleep(500);
	});

	it('runs', () => {
		expect(harper.isActive).toBe(true);
	});

	it('gives correct diagnostics for default config', () => {
		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(),
		);
	});

	it('marks error when set to British English', async () => {
		const config = workspace.getConfiguration('harper');
		await config.update('dialect', 'British', ConfigurationTarget.Workspace);
		// Wait for `harper-ls` to update diagnostics
		await sleep(300);

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics({
				message: 'Did you mean to spell “color” this way?',
				range: createRange(0, 41, 0, 46),
			}),
		);

		// Set config back to default value
		await config.update('dialect', 'American', ConfigurationTarget.Workspace);
	});
});
