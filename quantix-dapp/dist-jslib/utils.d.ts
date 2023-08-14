type CallbackFunctionElm = (element: HTMLElement, once: boolean) => void;
type CallbackFunctionElms = (elements: NodeListOf<HTMLElement>, once: boolean) => void;
export declare function waitForElm<T extends HTMLElement>(selector: string, debug: boolean, once: boolean, callback: CallbackFunctionElm): void;
export declare function waitForElms<T extends HTMLElement>(selector: string, debug: boolean, once: boolean, callback: CallbackFunctionElms): void;
export {};
