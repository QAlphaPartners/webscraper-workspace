import { invoke } from "@tauri-apps/api/tauri";

import { WebviewWindow } from '@tauri-apps/api/window';

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

export function createWebviewWindow(uniqueLabel: string, title: string, url: string) {
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
}