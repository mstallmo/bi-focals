
module Tabs = {
    [@bs.deriving abstract]
    type activeInfo = {
        tabId: int, 
        windowId: int
    };

    [@bs.deriving abstract]
    type details = {
        code: string
    };

    [@bs.scope ("chrome", "tabs", "onActivated" )] [@bs.val] external addListener : (activeInfo => unit) => unit = "";
    [@bs.scope ("chrome", "tabs")] [@bs.val] external executeScript : (int, details, (unit => unit)) => unit = "";
};

module Runtime = {
    [@bs.deriving abstract]
    type message = {
        content: string
    };
    type sender;
    type sendResponse;

    [@bs.scope ("chrome", "runtime", "onMessage")] [@bs.val]  external addListener : ((message, sender, sendResponse) => unit) => unit = "";
};


