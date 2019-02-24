const path = require("path");
const outputDir = path.join(__dirname, "build/");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProd = process.env.NODE_ENV === "production";

const copyPlugin = new CopyWebpackPlugin([
  { from: "manifest.json", to: "manifest.json" }
]);

const wasmPackPlugin = new WasmPackPlugin({
  crateDirectory: path.resolve(__dirname, ".")
});

module.exports = {
  entry: "./src/bootstrap.js",
  mode: isProd ? "production" : "development",
  watch: !isProd,
  devtool: "cheap-module-source-map",
  output: {
    path: outputDir,
    filename: "background.js"
  },
  resolve: {
    extensions: [".js", ".wasm"]
  },
  node: {
    fs: "empty"
  },
  plugins: [copyPlugin, wasmPackPlugin]
};
