// Import cash-dom
import { getCurrent } from '@tauri-apps/api/window';
import { scrape_urls } from '../url_scraper';
import { waitForElm } from '../utils';
import { Event as TauriEvent, listen } from '@tauri-apps/api/event';
import type { FataEvent, BomaEvent } from '../events/bindings/index.js';


console.log("I am finance-yahoo/scraper.ts")

let previousUrl: string = "null";

// add the function as an event listener for the load event
window.addEventListener("DOMContentLoaded", handleLoaded);

// Wait for the window to load
async function handleLoaded() {

    // To use it:
    waitForElm<HTMLDivElement>("#float-scrape-div").then(async floatDiv => {
        console.log("[waitForElm] found Element(float-scrape-div) is ready: ", floatDiv);

        floatDiv.innerHTML = "[finance-yahoo/scraper] float div with scraped web data in json from url=" + window.location.href;
        await scrape_urls()

        // Create an object literal with the required fields
        let fataEvent = {
            hub: "some hub name",
            topic: "some topic name",
            // Optionally, you can also add the label and data fields
            label: "some label",
            data: "some data from [scraper.ts]",
        } as FataEvent<any>; // Cast the object to the FataEvent type
        await getCurrent().emit("FataEvent", fataEvent);


        // listener has to be registered after emit event to backend!!! or else 
        // [Error] TypeError: listener.handler is not a function. (In 'listener.handler(eventData)', 'listener.handler' is undefined)
        const unlisten = await getCurrent().listen("BomaEvent",
            function (evt: TauriEvent<BomaEvent<any>>) {
                const bomaEvent = evt.payload;

                // Publish event to the given Hub
                if (bomaEvent.label != null) {
                    console.log("[finance-yahoo/scraper] listen got BomaEvent@float-scrape-div ", bomaEvent.topic, bomaEvent.label, bomaEvent.data);
                } else {
                    console.log("[finance-yahoo/scraper] listen got BomaEvent@float-scrape-div ", bomaEvent.topic, bomaEvent.data);
                }
            }
        );

        // Add an event listener for the window.onbeforeunload event
        window.onbeforeunload = function () {
            // Call unlisten() before closing the window
            unlisten();
            // Return null to prevent any confirmation dialog
            return null;
        };

        // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
        waitForElm<HTMLDivElement>("#jztable thead").then(thead => {
            console.log("[extractJjjzHistoryData] thead ready: ", thead);

            var tr_rows = thead.querySelectorAll("tr"); 
            // // The first child node is the table element, the second child node is the tbody element, and its child nodes are the tr elements
            console.log("[extractJjjzHistoryData] thead tr_rows",tr_rows)
            
        });


        // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
        waitForElm<HTMLDivElement>("#jztable tbody").then(jztable => {
            console.log("[extractJjjzHistoryData] tbody ready: ", jztable);

            var tr_rows = jztable.querySelectorAll("tr"); 
            // // The first child node is the table element, the second child node is the tbody element, and its child nodes are the tr elements
            console.log("[extractJjjzHistoryData] tbody tr_rows",tr_rows)

            tr_rows.forEach((tr,trkey,parent)=>{

                tr.querySelectorAll("td").forEach((td,tdkey,parent)=>{
                    console.log("tr["+trkey+"]"+"]td["+tdkey+"].text="+td.innerText)
                })
                
            })
            
        });

    });

};
