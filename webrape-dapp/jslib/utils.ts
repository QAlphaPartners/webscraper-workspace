// Define a type alias for the function parameter
type CallbackFunction = (element: HTMLElement, once: boolean) => void;

// Update the type of waitForElm to accept a function parameter and a boolean parameter
export function waitForElm<T extends HTMLElement>(selector: string, debug:boolean , callback: CallbackFunction, once: boolean): void {
    const observer = new MutationObserver((mutations) => {
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

        // Check if the element exists in the document
        const element = document.querySelector<T>(selector);
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
