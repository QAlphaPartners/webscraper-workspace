// Import the $() function from Cash JS
import $ from "cash-dom";

import * as Crawler from "./crawler"

import { invoke } from "@tauri-apps/api/tauri";

import { WebviewWindow } from '@tauri-apps/api/window';

import { emit, listen } from '@tauri-apps/api/event'
export interface BrowserOptions {
  dsn?: string;
  name?: string;
  version?: string;
  autoSessionTracking?: boolean;
  integrations?: string[];
}
//# sourceMappingURL=sdkinfo.d.ts.map
export interface Breadcrumb {
  type?: string;
  event_id?: string;
  category?: string;
  message?: string;
  data?: {
    [key: string]: any;
  };
  timestamp?: number;
}
export interface Event {
  event_id?: string;
  message?: string;
  timestamp?: number;
  start_timestamp?: number;
  platform?: string;
  logger?: string;
  server_name?: string;
  release?: string;
  dist?: string;
  environment?: string;
  request?: Request;
  transaction?: string;
  modules?: {
    [key: string]: string;
  };
  fingerprint?: string[];
  breadcrumbs?: Breadcrumb[];
  sdkProcessingMetadata?: {
    [key: string]: any;
  };
}
/**
 * A simple `beforeSend` that sends the envelope to the Rust process via Tauri invoke.
 */
export async function sendEventToRust(event: Event): Promise<Error | null> {
  // The Sentry Rust type de-serialisation doesn't like these in their
  // current state
  delete event.breadcrumbs;
  // These will be overridden in the host
  delete event.environment;
  // This isn't in the Rust types
  delete event.sdkProcessingMetadata;


  await invoke("plugin:fund_eastmoney|event", { event });

  // Stop events from being sent from the browser
  return null;
}

/**
 * A simple `beforeBreadcrumb` hook that sends the breadcrumb to the Rust process via Tauri invoke.
 */
export function sendBreadcrumbToRust(
  breadcrumb: Breadcrumb
): Breadcrumb | null {
  console.log("Conan A simple `beforeBreadcrumb` hook that sends the breadcrumb to the Rust process via Tauri invoke.")
  invoke("plugin:fund_eastmoney|breadcrumb", { breadcrumb });
  // We don't collect breadcrumbs in the renderer since they are passed to Rust
  return null;
}

/**
 * Default options for the Sentry browser SDK to pass events and breadcrumbs to
 * the Rust SDK.
 */
export const defaultOptions: BrowserOptions = {
  dsn: "https://123456@dummy.dsn/0",
  // We want to track app sessions rather than browser sessions
  autoSessionTracking: false,
};

export async function createWebviewWindow(uniqueLabel: string, title: string, url: string) {
  const webview = new WebviewWindow(uniqueLabel, {
    title: title,
    url: url,
  });
  webview.once('tauri://created', function () {
    console.log("webview window successfully created", webview.label)

  });
  webview.once('tauri://error', function (e) {
    console.log("an error happened creating the webview window", webview.label)
  });


  // listen to the DOM loaded event from the new window
  const unlisten = await listen<string>('BackendEventxyz', async (event) => {
    // do something when the DOM is loaded
    console.log('The BackendEventxyz is ready event', event)
    // await invoke("plugin:fund_eastmoney|open_link", { "url": "open from /plugins/fund-eastmoney/js/index.ts" });
  })


  webview.once('tauri://destroyed', function (e) {
    console.log("webview window successfully destroyed", webview.label);
    unlisten();
  });

}


export async function loadURL(url: string): Promise<void> {
  // Open the URL in the current window
  window.open(url, "_self");
}


// Define and export a function to start the crawler on a new tab
export function startCrawler(url: string): void {
  // Define an empty links array of HTMLAnchorElement objects
  const links: HTMLAnchorElement[] = [];
  // Open the link in a new tab
  const newTab = window.open(url, '_blank');
  // Check if the new tab is not null
  if (newTab) {
    console.log("startCrawler get newTab ", url);
    // Request an animation frame for the new tab
    newTab.requestAnimationFrame(() => {
      // Switch to the new tab
      newTab.focus();
      // Create a new MutationObserver to observe the DOM ready event
      const observer = new MutationObserver((mutations: MutationRecord[], observer: MutationObserver) => {
        // Check if the document is ready
        if (newTab.document.readyState === 'complete') {
          // Disconnect the observer
          observer.disconnect();
          // Get all the links from the new page and add them to the links array if they belong to the same domain as the original link
          $('a', newTab.document).each((i: number, el: Element) => {
            const link = el as HTMLAnchorElement;
            if (Crawler.isSameDomain(link.href, url)) {
              console.log("crawler got link", link.href)
              links.push(link);
            }
          });
          // Define a global index to keep track of the current link
          let currentIndex = 0;
          // Open the first link in the same tab that belongs to the same domain as the original link
          while (currentIndex < links.length && Crawler.isSameDomain(links[currentIndex].href, url)) {
            currentIndex++;
          }
          // Check if there are any links to visit within the same domain
          if (currentIndex < links.length) {
            Crawler.openLink(links[currentIndex], links, currentIndex);
          }
        }
      });
      // Start observing the document for changes in its ready state
      observer.observe(newTab.document, { attributes: true });
    });
  }
}
