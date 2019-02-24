Js.log("Hello, BuckleScript and Reason!");

// let model =
//   TensorFlowConverter.loadFrozenModel(
//     "https://some/path/to/model.json",
//     "https://some/path/to/weights",
//   );

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
  open Image;

  Js.log("got the message!");
  Js.log(request->contentGet);

  let imageSource = Parser.parseHtml(request->contentGet);

  let image = newImage();
  srcSet(image, imageSource);
  onLoad(
    image,
    () => {
      Js.log("Image loaded!");
      let inputTensor = TensorFlow.fromPixels(image);
      Js.log(inputTensor);
    },
  );

  Js.log(image);

  sendResponse(. imageSource);
});