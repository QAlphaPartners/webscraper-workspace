// Import cash-dom
import type { DataValue, FataEvent } from './event/bindings/index.js';

import { getCurrent } from '@tauri-apps/api/window';
export async function scrape_urls() {

    var parent_url = window.location.href;
    console.log("[url_scraper.ts] Element float-scrape-div scrape all the HTMLAnchorElement from page:", parent_url)

    // url网页刮取的网址
    // Create an object literal with the required fields
    let fataEvent = {
        hub: "some hub name",
        topic: "URLS_SCRAPED",
        // Optionally, you can also add the label and data fields
        label: "some label ",
        data: [{
            StringValue: { data: "some data from [url_scraper.ts] parent_url:" + parent_url, enalbe: true, }// This is the String value
        }], // Specify the type of the data field
    } as FataEvent<DataValue>; // Cast the object to unknown first, and then to FataEvent<DataValue[]>
    await getCurrent().emit("FataEvent", fataEvent);
}


