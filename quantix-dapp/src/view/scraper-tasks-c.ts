import { DCheckElement } from '@dom-native/ui';
import { all, BaseHTMLElement, customElement, elem, first, frag, getFirst, html, on, OnEvent, onEvent, onHub, position, scanChild, trigger } from 'dom-native';
import { ModelMutateResultData, ScrapeTask } from '../bindings/index.js';
import { scrapeTaskFmc } from '../model/index.js';
import { classable } from '../utils.js';



@customElement('scraper-tasks-c')
export class ScraperTasksComponent extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data

	#filter?: any
	set filter(f: any) { this.#filter = f; this.update() }
	// #endregion --- Data

	// #region    --- App Event
	// Create will refresh the full datagrid, in case of sort by name and such
	@onHub("Model", "scrape_task", "create")
	onScrapeTaskCreate() {
		this.update();
	}

	// Delete can be more selective in this case, will delete the row
	@onHub("Model", "scrape_task", "delete")
	onScrapeTaskDelete(data: ModelMutateResultData) {
		all(this, `scraper-task-row.${classable(data.id)}`).forEach(taskRowEl => {
			// Note: This will add the class in the taskRow, but the animations are on the cells
			//       as the scraper-task-row as the display: contents in the css 
			//       (to be transparent to the grid layout, hence, can't style it)
			taskRowEl.classList.add('anim-delete');

			// Note: Trick to start the dom deletion before the animation terminate to make it snapier 
			setTimeout(() => {
				taskRowEl.remove();
			}, 100);


			// Note: This is sementically correct way to delete it, on first transition end. 
			// taskRowEl.addEventListener('transitionend', (evt) => {
			//   // Note: Here we will get many events back (one per animated element and property)
			//   //       So, just delete on first.
			//   if (taskRowEl.isConnected) {
			//     taskRowEl.remove()
			//   }
			// });
		});
	}

	@onHub("Model", "scrape_task", "update")
	async onScrapeTaskUpdate(data: ModelMutateResultData) {
		console.log("[onScrapeTaskUpdate] Model:scrape_task:update data=", data);
		const newTask = await scrapeTaskFmc.get(data.id);
		all(this, `scraper-task-row.${classable(data.id)}`).forEach((taskEl) => (<ScraperTaskRow>taskEl).task = newTask);
	}
	// #endregion --- App Event

	// #region    --- UI Events
	@onEvent("pointerup", "scraper-task-row #scrape")
	async onActionScrape(evt: OnEvent) {

		let taskEl = evt.selectTarget.closest("scraper-task-row")!;
		let task_id = taskEl.task.id;
		console.log("[onActionScrape] evt=", evt, " task_id=", task_id);

		await scrapeTaskFmc.start_scrape(task_id);

	}

	@onEvent("pointerup", "scraper-task-row .show-more")
	onTaskShowMore(evt: OnEvent) {
		const MENU_CLASS = 'scraper-task-row-more-menu';

		// if already showing (will auto remove, but we do not want to popup it again)
		if (first(`body > menu-c.${MENU_CLASS}`)) return;

		const showMoreEl = evt.selectTarget;
		const task = showMoreEl.closest('scraper-task-row')!.task;

		const options = {
			'toggle': (task.done) ? "Mark Undone" : "Mark Done",
			'delete': elem("label", { class: "delete", $: { textContent: "Delete" } }),
		};

		// Show the meunu
		const menuEl = elem("menu-c", { "class": MENU_CLASS, $: { options } });
		document.body.appendChild(menuEl);
		on(menuEl, "SELECT", (evt: OnEvent<keyof typeof options>) => {
			if (evt.detail == 'delete') {
				scrapeTaskFmc.delete(task.id);
			} else if (evt.detail == 'toggle') {
				scrapeTaskFmc.update(task.id, { done: !task.done });
			}

		});
		position(menuEl, showMoreEl, { refPos: "BR", pos: "BL", gap: 4 });
	}

	@onEvent("CHANGE", "scraper-task-row d-check")
	onTaskCheckClick(evt: OnEvent<{ value: boolean }>) {
		let taskEl = evt.selectTarget.closest("scraper-task-row")!;
		let task_id = taskEl.task.id;
		let newDone = evt.detail.value;

		// Make sure to avoid infine loop 
		// (will get this event when changed by other mean as well)
		if (newDone !== taskEl.task.done) {
			scrapeTaskFmc.update(task_id, { done: evt.detail.value });
		}
	}
	// #endregion --- UI Events

	postDisplay() {
		this.update();
	}

	async update() {
		if (this.initialized) {
			const filter = {
				...this.#filter
			}
			const scrapeTasks = await scrapeTaskFmc.list(filter);

			const content = frag(scrapeTasks, task => elem('scraper-task-row', { $: { task } }));

			this.replaceChildren(content);

			if (scrapeTasks.length == 0) {
				trigger(this, "EMPTY");
			}
		}

	}
}

declare global {
	interface HTMLElementTagNameMap {
		'scraper-tasks-c': ScraperTasksComponent;
	}
}

const SCRAPER_TASK_ROW_HTML = html`
    <master>
        <logo class="H9lube">logo</logo>
        <div class="title">title</div>
        <ops>
        	<a class="url" href="#">https://socket.dev/npm/package/vite-plugin-template-html</a>
            <a id="scrape" class="action" href="#">爬取此页</a>
            <a id="ignore" class="action" href="#">忽略此页</a>
        </ops>
    </master>
    <a class="turl" href="https://socket.dev/npm/package/vite-plugin-template-html">
		<h3 class="LC20lb MBeuO">How to add async/await inside the listen events in tauri ...</h3>
	</a>
    <detail>
        <b class="info">2022年11月23日爬取</b> 
		<p>— Start using Socket to analyze vite-plugin-template-html and its 127 dependencies to secure your app from supply chain attacks.</p>
    </detail>
`;
// #region    --- scraper-task-row
@customElement('scraper-task-row')
export class ScraperTaskRow extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	#task!: ScrapeTask;
	set task(newTask: ScrapeTask) {
		const oldTask = this.#task as ScrapeTask | undefined;
		if (oldTask !== newTask) {
			this.#task = newTask;
			this.update(newTask, oldTask);
		}
	}
	get task() { return this.#task }
	// #endregion --- Data

	// #region    --- Key Els
	#titleEl!: HTMLElement;
	#urlEl!: HTMLElement;
	#infoEl!: HTMLElement;
	// #endregion --- Key Els

	init() {

		super.init();
		let content = document.importNode(SCRAPER_TASK_ROW_HTML, true);
		// Note: dom-native scanChild is a strict one fast pass child scanner. 
		//       Use all/first if needs to be more flexible. 
		[this.#titleEl, this.#infoEl] = getFirst(content, '.title', '.info');
		this.#urlEl = getFirst (content,  '.url');


		// FIXME: Check that order does not matter here.
		this.replaceChildren(content);
		this.update(this.#task);
	}

	update(newTask: ScrapeTask, oldTask?: ScrapeTask) {

		if (oldTask) {
			this.classList.remove(`${classable(oldTask.id)}`)
		}

		// if ready to be injected, we do the job
		if (newTask && this.#titleEl != null) {

			this.classList.add(`${classable(newTask.id)}`);

			this.#titleEl.textContent = newTask.title + " ~Title~ ";

			this.#urlEl.textContent= newTask.href;

			let info = newTask.ctime;
			info = info.substring(info.length - 5);
			this.#infoEl.textContent = `2022年11月23日爬取(ctime: ${info})`;
		}

	}
}
declare global {
	interface HTMLElementTagNameMap {
		'scraper-task-row': ScraperTaskRow;
	}
}
// #endregion --- scraper-task-row

