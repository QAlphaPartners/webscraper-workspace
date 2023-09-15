import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, customElement, elem, getFirst, html, onEvent, OnEvent } from 'dom-native';
import { Project } from '../bindings/index.js';
import { taskFmc } from '../model/index.js';

const HTML = html`
<header>
<h1> Fund Eastmoney Scraped Links </h1>
</header>
<section></section>
`;

@customElement('scraper-link-c')
export class ScraperLinkComponent extends BaseHTMLElement { // extends HTMLElement
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
		'scraper-link-c': ScraperLinkComponent;
	}
}
