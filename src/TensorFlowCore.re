type tensor;
type inferenceModel;
[@bs.send]
external execute: (inferenceModel, ~inputs: tensor, ~outputs: tensor) => tensor =
  "execute";