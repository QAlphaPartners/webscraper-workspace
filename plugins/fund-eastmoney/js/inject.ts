import * as Sentry from "./index";

import { invoke } from "@tauri-apps/api/tauri";
import * as Crawler from "./crawler"
// Import the $() function from Cash JS
import $ from "cash-dom";

import { UnlistenFn, emit, listen } from '@tauri-apps/api/event'

declare var __DEBUG__: boolean;

declare global {
  interface Window {
    Sentry: typeof Sentry;
  }
}

window.Sentry = Sentry;



console.log("[inject.ts] Start scraping web-data for url=", window.location.href)
// add the function as an event listener for the load event
window.addEventListener("DOMContentLoaded", handleLoaded);


function start() {
  var eastmoney_count = 0;
  var tauri_count = 0;
  // Sentry.init({
  //   ...defaultOptions,
  //   // We replace this with true or false before injecting this code into the browser
  //   debug: __DEBUG__,
  // });

  console.log("Start scraping web-data for url=", window.location.href)

  // check the current URL of the webview
  if (window.location.hostname === "fund.eastmoney.com"
    || window.location.hostname === "fundf10.eastmoney.com"
    || window.location.hostname === "finance.yahoo.com") {

    eastmoney_count += 1
    // execute the script only for this URL
    console.log("hello from ", window.location.href, "eastmoney_count=", eastmoney_count);
    // do something else

    // add the function as an event listener for the load event
    window.addEventListener("DOMContentLoaded", handleLoaded);

  } else if (window.location.href === "https://tauri.app/") {
    tauri_count += 1
    // execute the script only for this URL
    console.log("hello from ", window.location.href, "tauri_count=", tauri_count);
    // do something else
  }
}

// define a function that will run when any link is clicked
function handleClick(event: MouseEvent) {
  // prevent the default behavior of opening the link in a new window
  event.preventDefault();

  // get the event target
  const target = event.target;

  console.log("handleClick target is HTMLAnchorElement?", target, target instanceof HTMLAnchorElement)

  // check if the target is an <a> element
  if (target instanceof HTMLAnchorElement && target.tagName === "A") {
    // get the link URL from the href attribute
    const url = target.href;

    console.log("trying to window.open url _self", url)
    // Open the URL in the current window
    window.open(url, "_self");
  } else if (target instanceof HTMLElement && target.tagName === "LABEL") {

    // To use it: http://fundf10.eastmoney.com/jjjz_002190.html
    waitForElm<HTMLDivElement>("#jztable table").then(elm => {
      console.log("extractJjjzHistoryData for Element(jztable) is ready: ", $(elm).text());

      extractJjjzHistoryData(elm)

    });

  }
}

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
      console.log('The URL has changed from ' + previousUrl + ' to ' + window.location.href);
      // Update the previous URL
      previousUrl = window.location.href;
      floatDiv.innerHTML = "This is a float div with scraped web data in json from url=" + window.location.href;
    }
  });

  // Specify the mutation observer options
  const config = {
    subtree: true,
    childList: true
  };

  // Start observing changes in the document element
  observer.observe(document.documentElement, config);

  // add the function as an event listener for the click event on the document body
  document.body.addEventListener("click", handleClick);



  // To use it:
  waitForElm<HTMLDivElement>("#my-float-div").then(async elm => {
    console.log("Element(my-float-div) is ready and emit(DOMContentLoadedxxx): ", $(elm).text());

    var pp = await emit("DOMContentLoadedxxx", { loggedIn: true, token: 'authToken@waitForElm<HTMLDivElement>("#my-float-div")' });

    
    console.log("after emit(DOMContentLoadedxxx)");


    const unlisten = await listen("BackendEventxyz", (event) => {
      console.log("listen got BackendEventxyz@my-float-div ", event)
    })


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
    console.log("extractJjjzHistoryData for Element(jztable) is ready: ", $(elm).text());

    extractJjjzHistoryData(elm)

  });


  console.log("done created floatDiv when window.load and invoke(open_link) for url", window.location.href)
};

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

// Declare a generic type parameter T that extends HTMLElement
function waitForElm<T extends HTMLElement>(selector: string): Promise<T> {
  return new Promise(resolve => {
    // Check if the element already exists
    if ($(selector).length > 0) {
      return resolve($(selector)[0] as T);
    }

    // Create a mutation observer to watch for changes in the body
    const observer = new MutationObserver(mutations => {
      // Check if the element exists after each mutation
      if ($(selector).length > 0) {
        // Resolve the promise with the element and disconnect the observer
        resolve($(selector)[0] as T);
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

async function handleCrawling(url: string) {
  console.log("handleCrawling start to crawl url", url);

  await invoke("plugin:fund_eastmoney|open_link", { "url": url });

  // // Define an empty links array of HTMLAnchorElement objects
  // const links: HTMLAnchorElement[] = [];

  // // Get all the links from the new page and add them to the links array if they belong to the same domain as the original link
  // $('a', document).each((i: number, el: Element) => {
  //   const link = el as HTMLAnchorElement;
  //   if (Crawler.isSameDomain(link.href, url)) {
  //     console.log("handleCrawling 2 got link", link.href)
  //     links.push(link);
  //   }
  // });
  // // Define a global index to keep track of the current link
  // let currentIndex = 0;
  // // Open the first link in the same tab that belongs to the same domain as the original link
  // while (currentIndex < links.length && !Crawler.isSameDomain(links[currentIndex].href, url)) {
  //   links.shift()
  //   currentIndex++;
  // }
  // console.log("handleCrawling 3 got currentIndex,links.length", currentIndex, links.length)
  // // Check if there are any links to visit within the same domain
  // if (currentIndex < links.length) {
  //   console.log("handleCrawling 4 openLink", currentIndex)
  //   Crawler.openLink(links[currentIndex], links, currentIndex);
  // }

}
