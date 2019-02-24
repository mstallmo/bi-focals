Js.log("Hello, BuckleScript and Reason!");

Chrome.Tabs.addListener(active_info => {
  open Chrome.Tabs;

  Js.log(active_info);
  Js.log(active_info->tabIdGet);
  Js.log(active_info->windowIdGet);

  let scriptDetails =
    details(
      ~code=
        "chrome.runtime.sendMessage({content: document.body.innerHTML}, function(response) { console.log(response); });",
    );
  Chrome.Tabs.executeScript(active_info->tabIdGet, scriptDetails, () =>
    Js.log("Done!")
  );
});

Chrome.Runtime.addListener((request, _sender, sendResponse) => {
  open Chrome.Runtime;
  Js.log("got the message!");
  Js.log(request->contentGet);

  let output = Parser.parseHtml(request->contentGet);

  sendResponse(. output);
});