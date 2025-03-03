import { Lint, LocalLinter, Suggestion } from 'harper.js';
import { LintBox } from './Box';
import DataBlock from './DataBlock';
import { getRangeForTextSpan } from './domUtils';

export type EditContentCallback = (newContent: string) => void;

/**
 * Represents a rich text element on-screen.
 * It can either be a child element of a `DataBlock` or be the `DataBlock` itself.
 */
export default class RichText {
	private targetElement: Element;
	private parent: DataBlock;
	private editContent: EditContentCallback;

	constructor(
		targetElement: Element,
		parent: DataBlock,
		editContent: EditContentCallback
	) {
		this.targetElement = targetElement;
		this.parent = parent;
		this.editContent = editContent;
	}

	public getTargetElement(): Element {
		return this.targetElement;
	}

	public getTextContent(): string {
		return this.targetElement.textContent ?? '';
	}

	public computeLintBox(lint: Lint): LintBox[] {
		const text = this.targetElement.textContent;
		const span = lint.span();
		const range = getRangeForTextSpan(this.targetElement, span);
		const linter = new LocalLinter();

		if (range === null || text === null) {
			console.log('Could not locate range.');
			return [];
		}

		const targetRects = range.getClientRects();
		const container = DataBlock.getContainer();
		const contRect = container.getBoundingClientRect();

		const boxes: LintBox[] = [];

		for (const targetRect of targetRects) {
			boxes.push({
				x: targetRect.x - contRect.x,
				y: targetRect.y - contRect.y,
				width: targetRect.width,
				height: targetRect.height,
				lint,
				applySuggestion: async (sug: Suggestion) => {
					const fixed = await linter.applySuggestion(text, sug, span);

					this.editContent(fixed);
				},
			});
		}

		return boxes;
	}
}
