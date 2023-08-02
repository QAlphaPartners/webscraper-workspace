import { invoke } from "@tauri-apps/api/tauri";
let greetInputEl;
let greetMsgEl;
async function greet() {
    if (greetMsgEl && greetInputEl) {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsgEl.textContent = await invoke("greet", {
            name: greetInputEl.value,
        });
    }
}
async function createWindow() {
    await invoke("create_window");
}
window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");
    document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
        e.preventDefault();
        greet();
        createWindow();
    });
});
