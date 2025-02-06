import type { InitInput as BinaryInitInput } from 'wasm';

export type BinaryInit = BinaryInitInput | Promise<BinaryInitInput>;

export async function loadBinary(binary: BinaryInit) {
	const wasm = await import('wasm');
	await wasm.default({ module_or_path: binary });
	return wasm;
}

export { default as binary } from 'wasm/harper_wasm_bg.wasm?url';
export { default as inlinedBinary } from 'wasm/harper_wasm_bg.wasm?inline';
