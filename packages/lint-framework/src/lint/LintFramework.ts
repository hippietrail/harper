import computeLintBoxes from './computeLintBoxes';
import { isVisible } from './domUtils';
import Highlights from './Highlights';
import PopupHandler from './PopupHandler';
import type { UnpackedLint } from './unpackLint';

type ActivationKey = 'off' | 'shift' | 'control';

/** Events on an input (any kind) that can trigger a re-render. */
const INPUT_EVENTS = ['focus', 'keyup', 'paste', 'change', 'scroll'];
/** Events on the window that can trigger a re-render. */
const PAGE_EVENTS = ['resize', 'scroll'];

/** Orchestrates linting and rendering in response to events on the page. */
export default class LintFramework {
	private highlights: Highlights;
	private popupHandler: PopupHandler;
	private targets: Set<Node>;
	private scrollableAncestors: Set<HTMLElement>;
	private lintRequested = false;
	private renderRequested = false;
	private lastLints: { target: HTMLElement; lints: UnpackedLint[] }[] = [];

	/** The function to be called to re-render the highlights. This is a variable because it is used to register/deregister event listeners. */
	private updateEventCallback: () => void;

	/** Function used to fetch lints for a given text/domain. */
	private lintProvider: (text: string, domain: string) => Promise<UnpackedLint[]>;
	/** Actions wired by host environment (extension/app). */
	private actions: {
		ignoreLint?: (hash: string) => Promise<void>;
		getActivationKey?: () => Promise<ActivationKey>;
		openOptions?: () => Promise<void>;
		addToUserDictionary?: (words: string[]) => Promise<void>;
	};

	constructor(
		lintProvider: (text: string, domain: string) => Promise<UnpackedLint[]>,
		actions: {
			ignoreLint?: (hash: string) => Promise<void>;
			getActivationKey?: () => Promise<ActivationKey>;
			openOptions?: () => Promise<void>;
			addToUserDictionary?: (words: string[]) => Promise<void>;
		},
	) {
		this.lintProvider = lintProvider;
		this.actions = actions;
		this.highlights = new Highlights();
		this.popupHandler = new PopupHandler({
			getActivationKey: actions.getActivationKey,
			openOptions: actions.openOptions,
			addToUserDictionary: actions.addToUserDictionary,
		});
		this.targets = new Set();
		this.scrollableAncestors = new Set();
		this.lastLints = [];

		this.updateEventCallback = () => {
			this.update();
		};

		const timeoutCallback = () => {
			this.update();

			setTimeout(timeoutCallback, 100);
		};

		timeoutCallback();

		this.attachWindowListeners();
	}

	/** Returns the currents targets that are visible on-screen. */
	onScreenTargets(): Node[] {
		const onScreen = [] as Node[];

		for (const target of this.targets) {
			if (isVisible(target)) {
				onScreen.push(target);
			}
		}

		return onScreen;
	}

	async update() {
		this.requestRender();
		this.requestLintUpdate();
	}

	async requestLintUpdate() {
		if (this.lintRequested) {
			return;
		}

		// Avoid duplicate requests in the queue
		this.lintRequested = true;

		const lintResults = await Promise.all(
			this.onScreenTargets().map(async (target) => {
				if (!document.contains(target)) {
					this.targets.delete(target);
					return { target: null as HTMLElement | null, lints: [] as UnpackedLint[] };
				}

				const text =
					target instanceof HTMLTextAreaElement || target instanceof HTMLInputElement
						? target.value
						: target.textContent;

				if (!text || text.length > 120000) {
					return { target: null as HTMLElement | null, lints: [] as UnpackedLint[] };
				}

				const lints = await this.lintProvider(text, window.location.hostname);
				return { target: target as HTMLElement, lints };
			}),
		);

		this.lastLints = lintResults.filter((r) => r.target != null) as any;
		this.lintRequested = false;
		this.requestRender();
	}

	public async addTarget(target: Node) {
		if (!this.targets.has(target)) {
			this.targets.add(target);
			this.update();
			this.attachTargetListeners(target);
		}
	}

	public async removeTarget(target: HTMLElement) {
		if (this.targets.has(target)) {
			this.targets.delete(target);
			this.update();
			this.detachTargetListeners(target);
		} else {
			throw new Error('HTMLElement not added.');
		}
	}

	private attachTargetListeners(target: Node) {
		for (const event of INPUT_EVENTS) {
			target.addEventListener(event, this.updateEventCallback);
		}

		const observer = new MutationObserver(this.updateEventCallback);
		const config = { subtree: true, characterData: true };

		if ((target as any).tagName == undefined) {
			observer.observe((target as any).parentElement!, config);
		} else {
			observer.observe(target as Element, config);
		}

		const scrollableAncestors = getScrollableAncestors(target);

		for (const el of scrollableAncestors) {
			if (!this.scrollableAncestors.has(el as HTMLElement)) {
				this.scrollableAncestors.add(el as HTMLElement);
				(el as HTMLElement).addEventListener('scroll', this.updateEventCallback, {
					capture: true,
					passive: true,
				});
			}
		}
	}

	private detachTargetListeners(target: HTMLElement) {
		for (const event of INPUT_EVENTS) {
			target.removeEventListener(event, this.updateEventCallback);
		}
	}

	private attachWindowListeners() {
		for (const event of PAGE_EVENTS) {
			window.addEventListener(event, this.updateEventCallback);
		}
	}

	private requestRender() {
		if (this.renderRequested) {
			return;
		}

		this.renderRequested = true;

		requestAnimationFrame(() => {
			const boxes = this.lastLints.flatMap(({ target, lints }) =>
				target
					? lints.flatMap((l) =>
							computeLintBoxes(target, l as any, { ignoreLint: this.actions.ignoreLint }),
						)
					: [],
			);
			this.highlights.renderLintBoxes(boxes);
			this.popupHandler.updateLintBoxes(boxes);

			this.renderRequested = false;
		});
	}
}

/**
 * Returns all scrollable ancestor elements of a given element,
 * ordered from nearest to furthest (ending with the page scroller).
 */
function getScrollableAncestors(element: Node): Element[] {
	const scrollables: Element[] = [];
	const root = document.scrollingElement || document.documentElement;
	let parent = (element as any).parentElement;

	while (parent) {
		const style = window.getComputedStyle(parent);
		const { overflowY, overflowX } = style;
		const canScrollY = overflowY.includes('auto') || overflowY.includes('scroll');
		const canScrollX = overflowX.includes('auto') || overflowX.includes('scroll');

		if (canScrollY || canScrollX) {
			scrollables.push(parent);
		}
		parent = parent.parentElement;
	}

	// Always include the document scroller at the end
	if (root && scrollables[scrollables.length - 1] !== root) {
		scrollables.push(root);
	}

	return scrollables;
}
