// Import cash-dom
import $ from 'cash-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from '@tauri-apps/api/event';
console.log("I am finance-yahoo/crawler.ts");
// add the function as an event listener for the load event
window.addEventListener("DOMContentLoaded", handleLoaded);
// Wait for the window to load
async function handleLoaded() {
    // Create a new div element using JavaScript
    var floatDiv = document.createElement("div");
    // Set the div's id
    floatDiv.id = "my-float-div";
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
    // Store the previous URL
    let previousUrl = window.location.href;
    // Create a mutation observer
    const observer = new MutationObserver(() => {
        // Check if the current URL is different from the previous one
        if (window.location.href !== previousUrl) {
            // Do something when the URL changes
            console.log('[crawler_lazy.ts] The URL has changed from ' + previousUrl + ' to ' + window.location.href);
            // Update the previous URL
            previousUrl = window.location.href;
            floatDiv.innerHTML = "[crawler_lazy.ts] This is a float div with scraped web data in json from url=" + window.location.href;
        }
    });
    // Specify the mutation observer options
    const config = {
        subtree: true,
        childList: true
    };
    // Start observing changes in the document element
    observer.observe(document.documentElement, config);
    // To use it:
    waitForElm("#my-float-div").then(async (elm) => {
        console.log("[crawler_lazy.ts] Element(my-float-div) is ready and emit(DOMContentLoadedxxx): ", $(elm).text());
        var pp = await emit("DOMContentLoadedxxx", { loggedIn: true, token: 'authToken@waitForElm<HTMLDivElement>("#my-float-div")' });
        console.log("[crawler_lazy.ts] after emit(DOMContentLoadedxxx)", pp);
        const unlisten = await listen("BackendEventxyz", (event) => {
            console.log("[crawler_lazy.ts] listen got BackendEventxyz@my-float-div ", event);
        });
        // Add an event listener for the window.onbeforeunload event
        window.onbeforeunload = function () {
            // Call unlisten() before closing the window
            unlisten();
            // Return null to prevent any confirmation dialog
            return null;
        };
    });
    // // Select the node that will be observed for mutations
    // var targetNode = document.body;
    // // Options for the observer (which mutations to observe)
    // var config1 = { childList: true, subtree: true };
    // // Create an observer instance linked to the callback function
    // var observer1 = new MutationObserver((mutationsList) => {
    //   // Use traditional 'for loops' for IE 11
    //   for (var mutation of mutationsList) {
    //     if (mutation.type === 'childList') {
    //       // Check if the element with id "my-float-div" is removed
    //       var elm = document.getElementById("my-float-div");
    //       if (!document.body.contains(elm)) {
    //         console.log('The element with id "my-float-div" is removed from the DOM');
    //         // Stop listening to the "BackendEventxyz" event
    //         unlisten();
    //         // Stop observing
    //         observer.disconnect();
    //       }
    //     }
    //   }
    // });
    // // Start observing the target node for configured mutations
    // observer1.observe(targetNode, config1);
    // To use it:
    waitForElm("#pagebar").then(elm => {
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
                extractJjjzHistoryData(tableDiv);
            }
        });
    });
    // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
    waitForElm("#jztable table").then(elm => {
        console.log("[crawler_lazy.ts] extractJjjzHistoryData for Element(jztable) is ready: ", $(elm).text());
        extractJjjzHistoryData(elm);
    });
    console.log("done created floatDiv when window.load and invoke(open_link) for url", window.location.href);
    await invoke("plugin:fund_eastmoney|open_link", { "url": "open_link from /plugins/fund-eastmoney/js/crawler_lazy.ts" });
}
;
function extractJjjzHistoryData(elm) {
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
        var heads = row.find("th");
        if (heads.length > 0) {
            console.log(index, heads[0]?.innerText, heads[1]?.innerText, heads[2]?.innerText, heads[3]?.innerText);
        }
        var cells = row.find("td");
        if (cells.length > 0) {
            console.log(index, cells[0]?.innerText, cells[1]?.innerText, cells[2]?.innerText, cells[3]?.innerText);
        }
    });
}
// Declare a generic type parameter T that extends HTMLElement
function waitForElm(selector) {
    return new Promise(resolve => {
        // Check if the element already exists
        if ($(selector).length > 0) {
            return resolve($(selector)[0]);
        }
        // Create a mutation observer to watch for changes in the body
        const observer = new MutationObserver(() => {
            // Check if the element exists after each mutation
            if ($(selector).length > 0) {
                // Resolve the promise with the element and disconnect the observer
                resolve($(selector)[0]);
                observer.disconnect();
            }
        });
        // Start observing the body for childList and subtree changes
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });
}
