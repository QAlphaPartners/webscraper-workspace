// Declare a generic type parameter T that extends HTMLElement
export function waitForElm<T extends HTMLElement>(selector: string): Promise<T> {
    return new Promise(resolve => {

        let element = document.querySelector(selector);

        console.log("[waitForElm] selector:", selector, " and ", element);
        // Check if the element already exists
        if (element) {
            return resolve(element as T);
        }

        // Create a mutation observer to watch for changes in the body
        const observer = new MutationObserver(function (mutations) {
            // Loop through the mutations
            mutations.forEach(function (mutation) {
                // Check if any nodes were added
                if (mutation.addedNodes.length > 0) {
                    // Loop through the added nodes
                    mutation.addedNodes.forEach(function (node) {
                        // Log the node to the console
                        console.log("[utils.ts] MutationObserver addedNodes", node);
                    });
                }
            });

            let element = document.querySelector(selector);
            // Check if the element exists after each mutation
            if (element) {                // Resolve the promise with the element and disconnect the observer
                resolve(element as T)
                observer.disconnect();
            }
        });

        // Start observing the body for childList and subtree changes
        observer.observe(document, {
            childList: true,
            subtree: true
        });
    });
}

