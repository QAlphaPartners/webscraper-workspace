// Import cash-dom
import $ from 'cash-dom';

import { appWindow } from '@tauri-apps/api/window';
export async function scrape_urls() {

    var parent_url = window.location.href;
    console.log("[url_scraper.ts] Element my-float-div scrape all the HTMLAnchorElement from page:", parent_url)


    // url网页刮取的网址
    var pp = await appWindow.emit("URLS_SCRAPED", { loggedIn: true, token: 'authToken@scrape_urls:', parent_url: parent_url });
    pp

}


