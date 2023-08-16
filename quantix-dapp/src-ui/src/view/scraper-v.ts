import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, customElement, elem, getFirst, html, onEvent, OnEvent } from 'dom-native';
import { taskFmc } from '../model/index.js';

const HTML = html`
<header>
<h1> Scrape Web Data </h1>
<d-input class="search-url" placeholder="Search url to scrape"></d-input>
</header>
<section></section>
`;

@customElement('scraper-v')
export class ScraperView extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	// #endregion --- Data

	// #region    --- Key Els
	#contentEl!: HTMLElement
	#searchTaskDInputEl!: DInputElement
	// #endregion --- Key Els

	// #region    --- UI Events
	@onEvent("CHANGE", "d-input.search-url")
	onSearchChange(evt: OnEvent) {
		let search = (<DInputElement>evt.selectTarget).value.trim() as string;
		if (search.length > 0) {
			this.update({ title: { $contains: search } });
		} else {
			this.update();
		}
	}

	// #endregion --- UI Events

	init() {
		const content = document.importNode(HTML, true);

		[this.#contentEl,  this.#searchTaskDInputEl] =
			getFirst(content, "section", "d-input.search-url") as [HTMLElement,  DInputElement];

		this.replaceChildren(content);

		this.update()
	}

	async update(filter?: any) {
		if (this.#contentEl ) {
			// const taskDt = elem('tasks-dt', { $: { scraper_id: "123", filter } });
			// this.#contentEl.replaceChildren(taskDt);
		}
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'scraper-v': ScraperView;
	}
}
