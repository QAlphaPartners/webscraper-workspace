import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, customElement, elem, getFirst, html, onEvent, OnEvent } from 'dom-native';
import { taskFmc } from '../model/index.js';

const HTML = html`
<header>
<h1> Scrape Web Data Tasks </h1>
<d-input class="search-url" placeholder="Search url to scrape"></d-input>
</header>
<master>
	Section the component data here</br>
</master>
<detail>
	<scraper-link-c></scraper-link-c>
	<scraper-data-c></scraper-data-c>
</detail>
`;

const LINK_HEADER = html`
	<div class="th">Title </div>
	<div class="th">Info</div>
	<div class="th done">Done</div>
	<div class="th more">&nbsp;</div>
`

const LINK_ROW_HTML = html`
	<span class="title"></span>
	<span class="info"></span>
	<d-check class="done"></d-check>
	<d-ico class="show-more" name="ico-more"></d-ico>
`;


@customElement('scraper-v')
export class ScraperView extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	// #endregion --- Data

	// #region    --- Key Els
	#masterEl!: HTMLElement
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

		[this.#masterEl, this.#searchTaskDInputEl] =
			getFirst(content, "master", "d-input.search-url") as [HTMLElement, DInputElement];

		this.replaceChildren(content);

		this.update()
	}

	async update(filter?: any) {
		if (this.#masterEl) {
			const taskDt = elem('scraper-tasks-c');
			this.#masterEl.replaceChildren(taskDt);
		}
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'scraper-v': ScraperView;
	}
}
