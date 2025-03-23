import type { Extension, Uri } from 'vscode';

import { ConfigurationTarget, commands, workspace } from 'vscode';

import {
	activateHarper,
	compareActualVsExpectedDiagnostics,
	createExpectedDiagnostics,
	createRange,
	getActualDiagnostics,
	openFile,
	openUntitled,
	waitForHarperToActivate,
	waitForUpdatesFromConfigChange,
	waitForUpdatesFromDeletedFile,
	waitForUpdatesFromOpenedFile,
} from './helper';

describe('Integration >', () => {
	let harper: Extension<void>;
	let markdownUri: Uri;

	beforeAll(async () => {
		harper = await activateHarper();
		markdownUri = await openFile('integration.md');
		await waitForHarperToActivate();
	});

	it('runs', () => {
		expect(harper.isActive).toBe(true);
	});

	it('gives correct diagnostics for files', () => {
		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(
				{
					message: 'Did you mean to repeat this word?',
					range: createRange(2, 39, 2, 48),
				},
				{
					message: 'Did you mean to spell “errorz” this way?',
					range: createRange(2, 26, 2, 32),
				},
				{
					message: 'Did you mean to spell “realise” this way?',
					range: createRange(4, 26, 4, 33),
				},
			),
		);
	});

	it('gives correct diagnostics for untitled', async () => {
		const untitledUri = await openUntitled('Errorz');
		await waitForUpdatesFromOpenedFile();

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(untitledUri),
			createExpectedDiagnostics({
				message: 'Did you mean to spell “Errorz” this way?',
				range: createRange(0, 0, 0, 6),
			}),
		);
	});

	it('updates diagnostics on configuration change', async () => {
		const config = workspace.getConfiguration('harper.linters');
		await config.update('RepeatedWords', false, ConfigurationTarget.Workspace);
		await waitForUpdatesFromConfigChange();

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(
				{
					message: 'Did you mean to spell “errorz” this way?',
					range: createRange(2, 26, 2, 32),
				},
				{
					message: 'Did you mean to spell “realise” this way?',
					range: createRange(4, 26, 4, 33),
				},
			),
		);

		// Set config back to default value
		await config.update('RepeatedWords', true, ConfigurationTarget.Workspace);
	});

	it('accepts British spellings when dialect is set to British', async () => {
		const config = workspace.getConfiguration('harper');
		await config.update('dialect', 'British', ConfigurationTarget.Workspace);
		await waitForUpdatesFromConfigChange();

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(
				{
					message: 'Did you mean to repeat this word?',
					range: createRange(2, 39, 2, 48),
				},
				{
					message: 'Did you mean to spell “errorz” this way?',
					range: createRange(2, 26, 2, 32),
				},
			),
		);

		// Set config back to default value
		await config.update('dialect', 'American', ConfigurationTarget.Workspace);
	});

	it('updates diagnostics when files are deleted', async () => {
		const markdownContent = await workspace.fs.readFile(markdownUri);

		// Delete file through VS Code
		await commands.executeCommand('workbench.files.action.showActiveFileInExplorer');
		await commands.executeCommand('deleteFile');
		await waitForUpdatesFromDeletedFile();

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(),
		);

		// Restore and reopen deleted file
		await workspace.fs.writeFile(markdownUri, markdownContent);
		await openFile('integration.md');
		await waitForUpdatesFromOpenedFile();

		// Delete file directly
		await workspace.fs.delete(markdownUri);
		await waitForUpdatesFromDeletedFile();

		compareActualVsExpectedDiagnostics(
			getActualDiagnostics(markdownUri),
			createExpectedDiagnostics(),
		);

		// Restore and reopen deleted file
		await workspace.fs.writeFile(markdownUri, markdownContent);
		await openFile('integration.md');
	});
});
