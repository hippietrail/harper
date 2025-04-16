import * as path from 'node:path';
import * as fsPromises from 'node:fs/promises';
import { workspace } from 'vscode';
import * as child_process from 'child_process';

describe('VSIX Package >', () => {
    it('contains the font file', async () => {
        // Get the workspace folder path
        const workspacePath = workspace.workspaceFolders![0].uri.fsPath;
        
        // Get the extension root path by going up to the package.json location
        const extensionRootPath = path.join(workspacePath, '..', '..', '..');
        
        // Verify the VSIX package contents
        const vsixPath = path.join(extensionRootPath, 'harper-0.29.0.vsix');
        try {
            // Check if the VSIX file exists
            await fsPromises.access(vsixPath);
            
            // Create a temporary directory to extract the VSIX
            const tempDir = path.join(extensionRootPath, 'temp-vsix-extract');
            await fsPromises.mkdir(tempDir, { recursive: true });

            // Extract the VSIX using the system unzip command with -o flag to overwrite
            const unzipResult = child_process.spawnSync('unzip', [
                '-o', // Overwrite existing files without prompting
                vsixPath,
                '-d',
                tempDir
            ]);

            if (unzipResult.status !== 0) {
                throw new Error(`Failed to extract VSIX package: ${unzipResult.stderr.toString()}`);
            }

            // Verify the font file exists in the extracted VSIX
            const fontPath = path.join(tempDir, 'extension', 'media', 'harper.woff');
            try {
                await fsPromises.access(fontPath);
            } catch (error) {
                throw new Error(`Font file not found in VSIX package at ${fontPath}`);
            }

            // Clean up
            await fsPromises.rm(tempDir, { recursive: true, force: true });
        } catch (error) {
            throw new Error(`Failed to verify VSIX package contents: ${error}`);
        }
    });
});