
import {  emit,  } from '@tauri-apps/api/event'

declare var __DEBUG__: boolean;

console.log("[tauri-plugin-jsinject/inject.ts] inject common js into webview with url ", window.location.href)
window.addEventListener("DOMContentLoaded", async (event) => {
  await emit("InjectInited", { loggedIn: true, token: '[inject.ts] init', type: event.type });
})

