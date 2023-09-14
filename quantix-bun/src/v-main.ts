import { BaseHTMLElement, customElement, first, html, onEvent } from 'dom-native';

import typescriptLogo from './typescript.svg';
import viteLogo from '/vite.svg';

import { setupCounter } from './counter.ts'

@customElement('v-main') // same as customElements.define('v-main', IcoElement) 
class MainView extends BaseHTMLElement { // extends native HTMLElement

  #clickCount = 0; // private, transpiled by TypeScript.

  @onEvent('pointerup', '.hello-box')
  onHelloClick(evt: PointerEvent) {
    first(this, '.hello-box strong')!.textContent = `CLICKED ${++this.#clickCount}`;
  }

  init() { // called once on the first connectedCallback

    this.append(html`

      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="${viteLogo}" class="logo" alt="Vite logo" />
        </a>
        <a href="https://www.typescriptlang.org/" target="_blank">
          <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
        </a>
        <h1>Vite + TypeScript</h1>
        <div class="card">
          <button id="counter" type="button"></button>
        </div>
        <p class="read-the-docs">
          Click on the Vite and TypeScript logos to learn more
        </p>
      </div>

			<div class="hello-box">
				<c-ico href="#ico-thumb"></c-ico>
				Hello <strong>World</strong>a x23
			</div>
		`);

    setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)

  }

}