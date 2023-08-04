// Import cash-dom
import $ from 'cash-dom';
import { getCurrent } from '@tauri-apps/api/window';
import { scrape_urls } from '../url_scraper';
import { waitForElm } from '../utils';
import { Event as TauriEvent, listen } from '@tauri-apps/api/event';
import type { FataEvent,BomaEvent } from '../events/bindings/index.js';

// Specify the mutation observer options
const observerConfig = {
    subtree: true,
    childList: true
};

console.log("I am finance-yahoo/scraper.ts")

let previousUrl: string = "null";

// add the function as an event listener for the load event
window.addEventListener("DOMContentLoaded", handleLoaded);

// Wait for the window to load
async function handleLoaded() {
    // Create a new div element using JavaScript
    var floatDiv = createFloatScrapeDiv();

    // Create a mutation observer
    const observer = new MutationObserver(async () => {
        // Check if the current URL is different from the previous one
        if (window.location.href !== previousUrl) {
            // Do something when the URL changes
            console.log('[finance-yahoo/scraper] The URL has changed from ' + previousUrl + ' to ' + window.location.href);

            floatDiv.innerHTML = "[finance-yahoo/scraper] This is a float div with scraped web data in json from url=" + window.location.href;

            // Update the previous URL
            previousUrl = window.location.href;

        }
    });
    // Start observing changes in the document element
    observer.observe(document.documentElement, observerConfig);

    // To use it:
    waitForElm<HTMLDivElement>("#float-scrape-div").then(async elm => {

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


    });

    // To use it:
    waitForElm<HTMLDivElement>("#pagebar").then(elm => {
        console.log("Element(pagebar) is ready: ", $(elm).text());

        // Select the pagebtns' labels
        var labels = $(".pagebtns label");
        console.log(".pagebtns labels", labels, labels.length);

        // Attach a click handler
        labels.on("click", function (event) {
            // Do something when a label is clicked
            console.log(".pagebtns", event.target);

            var tableDiv = $("#jztable table");
            if (tableDiv instanceof HTMLDivElement) {
                extractJjjzHistoryData(tableDiv)
            }
        });

    });

    // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
    waitForElm<HTMLDivElement>("#jztable table").then(elm => {
        console.log("[finance-yahoo/scraper] extractJjjzHistoryData for Element(jztable) is ready: ", $(elm).text());

        extractJjjzHistoryData(elm)

    });

};

function createFloatScrapeDiv() {
    var floatDiv = document.createElement("div");

    // Set the div's id
    floatDiv.id = "float-scrape-div";

    // Set the div's content and style
    floatDiv.innerHTML = "This is a float div with scraped web data in json from url=" + window.location.href;
    floatDiv.style.position = "fixed"; // Change position to fixed
    floatDiv.style.bottom = "0px"; // Position it at the bottom
    floatDiv.style.left = "0px"; // Position it at the left
    floatDiv.style.width = "100%"; // Set the width to 100%
    floatDiv.style.height = "100px";
    floatDiv.style.backgroundColor = "red";
    floatDiv.style.color = "white";
    floatDiv.style.zIndex = "2147483647"; // Set the z-index to a high value

    // Add the float div to the body
    document.body.appendChild(floatDiv);
    return floatDiv;
}

function extractJjjzHistoryData(elm: HTMLDivElement) {

    // Assume you have a variable called tableDiv that is an HTMLDivElement
    // var tableDiv = document.getElementById("myTableDiv");
    var tableDiv = elm;

    // Wrap it in a cash object
    var table = $(tableDiv);

    // Select the table rows
    var rows = table.find("tr");

    // Iterate over the rows
    rows.each(function (index, element) {
        // Do something with each row
        var row = $(element);
        var heads = row.find("th")
        if (heads.length > 0) {
            console.log(index, heads[0]?.innerText, heads[1]?.innerText, heads[2]?.innerText, heads[3]?.innerText);
        }

        var cells = row.find("td")
        if (cells.length > 0) {
            console.log(index, cells[0]?.innerText, cells[1]?.innerText, cells[2]?.innerText, cells[3]?.innerText);
        }
    });
}
