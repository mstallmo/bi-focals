[@bs.deriving abstract]
type image = {
  mutable src: string,
  mutable alt: string,
};

[@bs.new] external newImage: unit => image = "Image";
[@bs.send] external onLoad: (image, unit => unit) => unit = "onload";