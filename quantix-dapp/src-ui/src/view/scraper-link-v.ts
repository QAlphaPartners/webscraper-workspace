import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, customElement, elem, getFirst, html, onEvent, OnEvent } from 'dom-native';
import { Project } from '../bindings/index.js';
import { taskFmc } from '../model/index.js';

const HTML = html`
<header>
<h1> Scraper Link </h1>
</header>
<section></section>
`;

@customElement('scraper-link-v')
export class ScraperLinkView extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	// #endregion --- Data

	// #region    --- Key Els
	#titleEl!: HTMLElement
	#contentEl!: HTMLElement
	#newTaskDInputEl!: DInputElement
	#searchTaskDInputEl!: DInputElement
	// #endregion --- Key Els

	// #region    --- UI Events
	
	// #endregion --- UI Events

	init() {
		const content = document.importNode(HTML, true);

		this.replaceChildren(content);

		this.update()
	}

	async update(filter?: any) {
		
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'scraper-link-v': ScraperLinkView;
	}
}
