import * as Sentry from "./";

declare var __DEBUG__: boolean;

declare global {
  interface Window {
    Sentry: typeof Sentry;
  }
}

window.Sentry = Sentry;
var eastmoney_count = 0;
var tauri_count = 0;
// Sentry.init({
//   ...defaultOptions,
//   // We replace this with true or false before injecting this code into the browser
//   debug: __DEBUG__,
// });

console.log("You call me inject.min.js here for url=", window.location.href)

// check the current URL of the webview
if (window.location.href === "https://fund.eastmoney.com/" || window.location.href === "https://finance.yahoo.com/") {
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




// define a function that will run when any link is clicked
function handleClick(event: MouseEvent) {
  // prevent the default behavior of opening the link in a new window
  event.preventDefault();

  // get the event target
  const target = event.target;

  console.log("handleClick target=", target)

  // check if the target is an <a> element
  if (target instanceof HTMLLinkElement && target.tagName === "A") {
    // get the link URL from the href attribute
    const url = target.href;

    // use window.eval to change the window location to the link URL
    window.eval(`window.location.replace('${url}')`);
  }
}

// Wait for the window to load
function handleLoaded() {

  console.log("try to create floatDiv when window.load", window.location.href)
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

};
