
import { getCurrent } from '@tauri-apps/api/window';

import type { FataEvent,  DataValue } from 'webrape-events';

declare var __DEBUG__: boolean;

let previousUrl: string = "null";

console.log("[inject.ts] inject common js into webview with url ", window.location.href)
window.addEventListener("DOMContentLoaded", async (event) => {

  // Check if the current URL is different from the previous one
  if (window.location.href !== previousUrl) {
    if (window.location.href !== "about:blank") {

      // Create an object literal with the required fields
      let fataEvent = {
        hub: "some hub name",
        topic: "some topic name",
        // Optionally, you can also add the label and data fields
        label: "some label",
        data: [{
          StringValue: {
            data: "some data from [inject.ts] init " + event.type,
            enalbe: true,
          }// This is the StringValue
        }]
      } as FataEvent<DataValue>; // Cast the object to unknown first, and then to FataEvent<DataValue[]>
      await getCurrent().emit("FataEvent", fataEvent);
    }

    // Update the previous URL
    previousUrl = window.location.href;
  }

  // Create a new observer
  var observer = new MutationObserver(function (mutations) {
    // Loop through the mutations
    mutations.forEach(function (mutation) {
      // Check if any nodes were added
      if (mutation.addedNodes.length > 0) {
        // Loop through the added nodes
        mutation.addedNodes.forEach(function (node) {
          // Log the node to the console
          console.log("[inject.ts] DOMContentLoaded MutationObserver addedNodes", node);
        });
      }
    });
  });

  // Specify the options for the observer
  var options = {
    childList: true, // Observe child nodes
    subtree: true // Observe the subtree of the target node
  };

  createFloatScrapeDiv();
  // Start observing the body element
  observer.observe(document.body, options);

})



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

