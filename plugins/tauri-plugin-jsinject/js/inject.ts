
import { getCurrent } from '@tauri-apps/api/window';
import type { FataEvent } from '../../../webrape-dapp/jslib/events/bindings/index.js';

declare var __DEBUG__: boolean;

let previousUrl: string = "null";

console.log("[tauri-plugin-jsinject/inject.ts] inject common js into webview with url ", window.location.href)
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
        data: "some data from [inject.ts] init " + event.type,
      } as FataEvent<any>; // Cast the object to the FataEvent type
      await getCurrent().emit("FataEvent", fataEvent);
    }

    // Update the previous URL
    previousUrl = window.location.href;
  }

})

