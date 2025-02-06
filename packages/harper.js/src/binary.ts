import type { InitInput as BinaryInitInput } from 'harper-wasm';
import * as wasm from 'harper-wasm';

export type BinaryInit = BinaryInitInput | Promise<BinaryInitInput>;

export async function loadBinary(binary: BinaryInit): Promise<typeof wasm> {
	const exports = await import('harper-wasm');
	await exports.default({ module_or_path: binary });
	return exports;
}

export { default as binary } from 'harper-wasm/harper_wasm_bg.wasm?url';
export { default as inlinedBinary } from 'harper-wasm/harper_wasm_bg.wasm?inline';
