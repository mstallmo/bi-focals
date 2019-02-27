const path = require("path");
const outputDir = path.join(__dirname, "build/");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProd = process.env.NODE_ENV === "production";

const copyPlugin = new CopyWebpackPlugin([
  { from: "manifest.json", to: "manifest.json" }
]);

module.exports = {
  entry: {
    background: "./src/background.js",
    contentScript: "./src/contentScript.ts"
  },
  mode: isProd ? "production" : "development",
  watch: !isProd,
  devtool: "cheap-module-source-map",
  output: {
    path: outputDir,
    filename: "[name].js"
  },
  resolve: {
    extensions: [".js", ".wasm", ".ts"]
  },
  node: {
    fs: "empty"
  },
  plugins: [copyPlugin],
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: "ts-loader",
        exclude: /node_modules/
      }
    ]
  }
};
