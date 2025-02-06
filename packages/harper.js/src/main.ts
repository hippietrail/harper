export { Lint, Span, Suggestion, SuggestionKind } from 'harper-wasm';
export { default as LocalLinter } from './LocalLinter';
export { default as WorkerLinter } from './WorkerLinter';
export type { default as Linter } from './Linter';
export { type BinaryInit, binary, inlinedBinary, loadBinary } from './binary';

/** A linting rule configuration dependent on upstream Harper's available rules.
 * This is a record, since you shouldn't hard-code the existence of any particular rules and should generalize based on this struct. */
export type LintConfig = Record<string, boolean | undefined>;

/** The option used to configure the parser for an individual linting operation. */
export type LintOptions = {
	/** The markup language that is being passed. Defaults to `markdown`. */
	language?: 'plaintext' | 'markdown';
};
