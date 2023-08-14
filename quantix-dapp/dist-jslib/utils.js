// Update the type of waitForElm to accept a function parameter and a boolean parameter
export function waitForElm(selector, debug, once, callback) {
    const observer = new MutationObserver((mutations) => {
        // Loop through the mutations
        mutations.forEach(function (mutation) {
            // Check if any nodes were added
            if (mutation.addedNodes.length > 0) {
                if (debug) {
                    // Loop through the added nodes
                    mutation.addedNodes.forEach(function (node) {
                        // Log the node to the console
                        console.log("[utils.ts] MutationObserver addedNodes", node);
                    });
                }
            }
        });
        // Check if the element exists in the document
        const element = document.querySelector(selector);
        if (element) {
            // Call the callback function with the element and the once parameter
            callback(element, once);
            // Check if the once parameter is true
            if (once) {
                // Stop observing
                observer.disconnect();
            }
        }
    });
    // Start observing
    observer.observe(document.body, {
        childList: true,
        subtree: true,
    });
}
// Update the type of waitForElm to accept a function parameter and a boolean parameter
export function waitForElms(selector, debug, once, callback) {
    const observer = new MutationObserver((mutations) => {
        // Loop through the mutations
        mutations.forEach(function (mutation) {
            // Check if any nodes were added
            if (mutation.addedNodes.length > 0) {
                if (debug) {
                    // Loop through the added nodes
                    mutation.addedNodes.forEach(function (node) {
                        // Log the node to the console
                        console.log("[utils.ts] MutationObserver addedNodes", node);
                    });
                }
            }
        });
        // Check if the element exists in the document
        const elements = document.querySelectorAll(selector);
        if (elements) {
            // Call the callback function with the element and the once parameter
            callback(elements, once);
            // Check if the once parameter is true
            if (once) {
                // Stop observing
                observer.disconnect();
            }
        }
    });
    // Start observing
    observer.observe(document.body, {
        childList: true,
        subtree: true,
    });
}
