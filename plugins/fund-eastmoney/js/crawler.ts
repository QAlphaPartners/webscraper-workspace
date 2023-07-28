// Import cash-dom
import $ from 'cash-dom';

import { invoke } from "@tauri-apps/api/tauri";

// Define a function to get the domain name from a URL
function getDomain(url: string): string | null {
    // Use a regular expression to match the domain name
    const match = url.match(/^(?:https?:\/\/)?(?:[^@\n]+@)?(?:www\.)?([^:\/\n?]+)/);
    // Return the first group if found, otherwise return null
    return match ? match[1] : null;
}

// Define a function to check if two URLs belong to the same domain
export function isSameDomain(url1: string, url2: string): boolean {
    // Get the domain names from the URLs
    const domain1 = getDomain(url1);
    const domain2 = getDomain(url2);
    // Return true if they are equal, otherwise return false
    return domain1 === domain2;
}

// Define a function to open a link in the same tab
export async function openLink(link: HTMLAnchorElement, links: HTMLAnchorElement[], currentIndex: number): Promise<void> {
    // Navigate to the link
    window.location.href = link.href;
    await invoke("plugin:fund_eastmoney|open_link", { "url": link.href });
    // You are right, setTimeout() is not very reliable or efficient for waiting for the page to load, 
    // as it depends on the network speed and the browser performance. A better alternative is to use 
    // the window.requestAnimationFrame() method, which is a browser API that allows you to execute a callback function 
    // before the next repaint of the screen. This way, you can ensure that the page is fully loaded and ready for manipulation 
    // before running your code.
    window.requestAnimationFrame(() => {
        // Create a new MutationObserver to observe the DOM ready event
        const observer = new MutationObserver((mutations: MutationRecord[], observer: MutationObserver) => {
            // Check if the document is ready
            if (window.document.readyState === 'complete') {
                // Disconnect the observer
                observer.disconnect();
                // Get all the links from the new page
                const newLinks = $('a', window.document).map((i: number, el: Element) => el as HTMLAnchorElement).get();
                // Add them to the global array if they are not already there and belong to the same domain as the original link
                for (let _newLink of newLinks) {
                    let newLink = _newLink as HTMLAnchorElement
                    if (!links.includes(newLink) && isSameDomain(newLink.href, link.href)) {
                        links.push(newLink);
                    }
                }
                // Increment the global index
                currentIndex++;
                // Check if there are more links to visit within the same domain
                while (currentIndex < links.length && !isSameDomain(links[currentIndex].href, link.href)) {
                    links.shift();
                    currentIndex++;
                }
                // Check if there are more links to visit within the same domain
                if (currentIndex < links.length) {
                    // Open the next link in the same tab
                    // openLink(links[currentIndex], links, currentIndex);

                }
            }
        });
        // Start observing the document for changes in its ready state
        observer.observe(window.document, { attributes: true });
    });
}
