import { waitForElms } from './utils';
import type { DataValue, FataEvent } from 'webrape-events';

import { getCurrent } from '@tauri-apps/api/window';
export async function scrape_urls() {

    var parent_url = window.location.href;
    console.log("[url_scraper.ts] Element float-scrape-div scrape all the HTMLAnchorElement from page:", parent_url)

    waitForElms("a", true, true, async (elements: NodeListOf<HTMLElement>) => {
        var links = Array<DataValue>();
        console.log("[url_scraper.ts] waitForElms links ", elements)
        elements.forEach((elm) => {
            if (elm instanceof HTMLAnchorElement) {
                var elm_ = elm as HTMLAnchorElement;
                if (elm_.href && elm_.href.startsWith("http")) {
                    var dataValue: DataValue = {
                        HTMLAnchorElementValue: { title: elm.title, href: elm.href, inner_text: elm.innerText.replace("\n", ""), scraped_date: BigInt(-1) }
                    }
                    links.push(dataValue);
                }
            };
        })


        // url网页刮取的网址
        // Create an object literal with the required fields
        let fataEvent = {
            hub: "links hub",
            topic: "URLS_SCRAPED",
            // Optionally, you can also add the label and data fields
            label: "all http HTMLAnchorElement elements",
            data: links
        } as FataEvent<DataValue>; // Cast the object to unknown first, and then to FataEvent<DataValue[]>
        await getCurrent().emit("FataEvent", fataEvent);
    })

}


