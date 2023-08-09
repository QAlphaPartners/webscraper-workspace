import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}


async function start_scrape(url: string, to_crawl: boolean) {
  await invoke("start_scrape", { url: url, toCrawl: to_crawl });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#fundeastmoney")?.addEventListener("click", (e) => {
    e.preventDefault();
    let link = e.target as HTMLAnchorElement
    start_scrape(link.href, true);
  });

  document.querySelector("#fundf10")?.addEventListener("click", (e) => {
    e.preventDefault();
    let link = e.target as HTMLAnchorElement
    start_scrape(link.href, false);
  });



  document.querySelector("#financeyahoo")?.addEventListener("click", (e) => {
    e.preventDefault();
    let link = e.target as HTMLAnchorElement
    start_scrape(link.href, true);
  });

});
