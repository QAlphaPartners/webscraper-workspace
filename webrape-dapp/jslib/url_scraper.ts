// Import cash-dom
import $ from 'cash-dom';

import { appWindow } from '@tauri-apps/api/window';
export async function scrape_urls(url: string) {

    console.log("[url_scraper.ts] Element my-float-div scrape all the HTMLAnchorElement from page:", url)


    // url网页刮取的网址
    var pp = await appWindow.emit("URLS_SCRAPED", { loggedIn: true, token: 'authToken@scrape_urls:' + url });
    pp
    
}


