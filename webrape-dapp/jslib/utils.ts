// Import cash-dom
import $ from 'cash-dom';


// Declare a generic type parameter T that extends HTMLElement
export function waitForElm<T extends HTMLElement>(selector: string): Promise<T> {
    return new Promise(resolve => {
        console.log("[waitForElm] selector:", selector, " and ", $(selector));
        // Check if the element already exists
        if ($(selector) && $(selector).length > 0) {
            return resolve($(selector)[0] as T);
        }

        // Create a mutation observer to watch for changes in the body
        const observer = new MutationObserver(() => {
            // Check if the element exists after each mutation
            if ($(selector) && $(selector).length > 0) {
                // Resolve the promise with the element and disconnect the observer
                resolve($(selector)[0] as T);
                observer.disconnect();
            }
        });

        // Start observing the body for childList and subtree changes
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });
}