// Import cash-dom
import type { FataEvent } from './events/bindings/index.js';

import { getCurrent } from '@tauri-apps/api/window';
export async function scrape_urls() {

    var parent_url = window.location.href;
    console.log("[url_scraper.ts] Element float-scrape-div scrape all the HTMLAnchorElement from page:", parent_url)

    // url网页刮取的网址
    // Create an object literal with the required fields
    let event = {
        hub: "some hub name",
        topic: "URLS_SCRAPED",
        // Optionally, you can also add the label and data fields
        label: "some label ",
        data: "some data from [url_scraper.ts] parent_url:" + parent_url,
    } as FataEvent<any>; // Cast the object to the FataEvent type
    await getCurrent().emit("FataEvent", event);


}


