// Import cash-dom
import { getCurrent } from '@tauri-apps/api/window';
import { scrape_urls } from '../url_scraper';
import { waitForElm } from '../utils';
import { Event as TauriEvent, listen } from '@tauri-apps/api/event';
import type { FataEvent, BomaEvent, DataValue } from 'webrape-events';


// add the function as an event listener for the load event
window.addEventListener("DOMContentLoaded", handleLoaded);

// Wait for the window to load
async function handleLoaded() {

    console.log("[scraper.ts] DOMContentLoaded handleLoaded");

    // To use it:
    waitForElm("#float-scrape-div", true, true, async (floatDiv: HTMLElement) => {
        console.log("[waitForElm] found Element(float-scrape-div) is ready: ", floatDiv);

        floatDiv.innerHTML = "[fund-eastmoney/scraper] float div with scraped web data in json from url=" + window.location.href;
        await scrape_urls()

        // Create an object literal with the required fields
        let fataEvent = {
            hub: "some hub name",
            topic: "some topic name",
            // Optionally, you can also add the label and data fields
            label: "some label",
            data: [{ StringValue: { data: "some data from [fund-eastmoney/scraper.ts]", enalbe: false } }], // Specify the type of the data field
        } as FataEvent<DataValue>; // Cast the object to unknown first, and then to FataEvent<DataValue[]>
        await getCurrent().emit("FataEvent", JSON.stringify(fataEvent));

        // listener has to be registered after emit event to backend!!! or else 
        // [Error] TypeError: listener.handler is not a function. (In 'listener.handler(eventData)', 'listener.handler' is undefined)
        const unlisten = await getCurrent().listen("BomaEvent",
            function (evt: TauriEvent<BomaEvent<any>>) {
                const bomaEvent = evt.payload;

                // Publish event to the given Hub
                if (bomaEvent.label != null) {
                    console.log("[fund-eastmoney/scraper] listen got BomaEvent@float-scrape-div ", bomaEvent.topic, bomaEvent.label, bomaEvent.data);
                } else {
                    console.log("[fund-eastmoney/scraper] listen got BomaEvent@float-scrape-div ", bomaEvent.topic, bomaEvent.data);
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


    });


    // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
    waitForElm("#jztable thead", true, false, (thead: HTMLElement) => {
        console.log("[extractJjjzHistoryData] thead ready: ", thead);

        var tr_rows = thead.querySelectorAll("tr");
        // // The first child node is the table element, the second child node is the tbody element, and its child nodes are the tr elements
        console.log("[extractJjjzHistoryData] thead tr_rows", tr_rows)

    });


    // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
    waitForElm("#jztable tbody", false, false, async (jztable: HTMLElement) => {
        console.log("[extractJjjzHistoryData] tbody ready: ", jztable);

        var tr_rows = jztable.querySelectorAll("tr");
        // // The first child node is the table element, the second child node is the tbody element, and its child nodes are the tr elements
        console.log("[extractJjjzHistoryData] 净值日期	单位净值	累计净值	日增长率", tr_rows)

        var netValues = Array<DataValue>();

        tr_rows.forEach((tr, trkey, parent) => {
            var tds = tr.querySelectorAll("td");
            var dataValue: DataValue = {
                FundNetValue: {
                    date: tds.item(0).innerText,
                    unit_value: Number(tds.item(1).innerText),
                    cumulate_value: Number(tds.item(2).innerText),
                    daily_rate: Number(tds.item(3).innerText.replace("%", ""))
                }
            }
            netValues.push(dataValue);

            console.log("tr[" + trkey + "]" + " td[0]=" + tds.item(0).innerText + " td[1]=" + tds.item(1).innerText + " td[2]=" + tds.item(2).innerText + " td[3]=" + tds.item(3).innerText)
        })

        // Create an object literal with the required fields
        let fataEvent = {
            hub: "fund-eastmoney",
            topic: "fund-net-value",
            // Optionally, you can also add the label and data fields
            label: "some label",
            data: netValues,
        } as FataEvent<DataValue>; // Cast the object to the FataEvent type
        await getCurrent().emit("FataEvent", fataEvent);
    });

};
