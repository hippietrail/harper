import type { Lint, Span, Suggestion } from 'wasm';
import { SuggestionKind } from 'wasm';
import Linter from './Linter';
import LocalLinter from './LocalLinter';
import WorkerLinter from './WorkerLinter';

export { LocalLinter, WorkerLinter, SuggestionKind };
export type { Linter, Lint, Span, Suggestion };

export type LintConfig = Record<string, boolean | undefined>;
