import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { workspace } from 'vscode';

describe('Font >', () => {
	it('font file is included in extension package', async () => {
		// Get the workspace folder path
		const workspacePath = workspace.workspaceFolders![0].uri.fsPath;

		// Get the extension root path by going up to the package.json location
		const extensionRootPath = path.join(workspacePath, '..', '..', '..');

		// Check if the media directory exists in the extension root
		const mediaPath = path.join(extensionRootPath, 'media');
		try {
			await fs.access(mediaPath);
		} catch (error) {
			throw new Error(
				`Media directory not found at ${mediaPath}. This indicates that the media directory may not have been properly included in the extension package.`,
			);
		}

		// Check if the font file exists in the media directory
		const fontPath = path.join(mediaPath, 'harper.woff');
		try {
			await fs.access(fontPath);
		} catch (error) {
			throw new Error(
				`Font file not found at ${fontPath}. This indicates that the font file may not have been properly included in the extension package.`,
			);
		}
	});
});
