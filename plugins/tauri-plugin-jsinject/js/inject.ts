
import { emit, } from '@tauri-apps/api/event'

declare var __DEBUG__: boolean;

let previousUrl: string = "null";

console.log("[tauri-plugin-jsinject/inject.ts] inject common js into webview with url ", window.location.href)
window.addEventListener("DOMContentLoaded", async (event) => {

  // Check if the current URL is different from the previous one
  if (window.location.href !== previousUrl) {

    await emit("InjectInited", { logged_in: true, token: '[inject.ts] init', type: event.type, url: window.location.href });

    // Update the previous URL
    previousUrl = window.location.href;
  }

})

