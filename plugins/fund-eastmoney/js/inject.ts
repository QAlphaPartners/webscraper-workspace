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
window.addEventListener("DOMContentLoaded", async (event) => {
  await emit("InjectInited", { logged_in: true, token: '[inject.ts] init', type: event.type });
})