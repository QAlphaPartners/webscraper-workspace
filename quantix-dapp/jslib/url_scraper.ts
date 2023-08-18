import { waitForElms } from './utils';
import type { DataValue, FataEvent } from 'webrape-events';

import { getCurrent } from '@tauri-apps/api/window';
export async function scrape_urls() {

    var parent_url = window.location.href;
    console.log("[url_scraper.ts] to process page:", parent_url)

    waitForElms("a", false, false, async (elements: NodeListOf<HTMLElement>) => {
        var links = Array<DataValue>();
        console.log("[url_scraper.ts] waitForElms links ", elements)
        elements.forEach((elm, key) => {
            if (elm instanceof HTMLAnchorElement) {
                var elm_ = elm as HTMLAnchorElement;
                if (elm_.href && elm_.href.startsWith("http")) {
                    var dataValue: DataValue = {
                        HTMLAnchorElementValue: { title: elm.title, href: elm.href, inner_text: elm.innerText.replace("\n", ""), scraped_date: 0 }
                    }
                    links.push(dataValue);
                }
            };
        })

        if (links.length > 0) {
            var size = 10;
            var splitArray = Array.from(
                new Array(Math.ceil(links.length / size)),
                (_, i) => links.slice(i * size, i * size + size)
            );

            splitArray.forEach(async function (subarray, index, array) {
                // TODO: very important, event can not be too large to send, or else will block the ipc channel!!!
                console.log("Subarray " + index + " of " + array.length + ": " + subarray);
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
            });


        }
    })

}


