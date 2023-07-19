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

console.log("You call me inject.min.js here for url=",window.location.href )

// check the current URL of the webview
if (window.location.href === "https://fund.eastmoney.com/") {
  eastmoney_count += 1
  // execute the script only for this URL
  console.log("hello from ", window.location.href, "eastmoney_count=",eastmoney_count);
  // do something else
}else if (window.location.href === "https://tauri.app/") {
  tauri_count += 1
  // execute the script only for this URL
  console.log("hello from ", window.location.href, "tauri_count=",tauri_count);
  // do something else
}
