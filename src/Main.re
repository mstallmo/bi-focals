Js.log("Hello, BuckleScript and Reason!");

Chrome.Tabs.addListener(active_info => {
  open Chrome.Tabs;

  Js.log(active_info);
  Js.log(active_info->tabIdGet);
  Js.log(active_info->windowIdGet);

  let scriptDetails = details(~file="contentScript.js", ~code=None);
  Chrome.Tabs.executeScript(active_info->tabIdGet, scriptDetails, () =>
    Js.log("Done!")
  );
});