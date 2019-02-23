const path = require("path");
const outputDir = path.join(__dirname, "build/");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const isProd = process.env.NODE_ENV === "production";

const copyPlugin = new CopyWebpackPlugin([
    {from: 'manifest.json', to: 'manifest.json'}
])

module.exports = {
    entry: "./src/Main.bs.js",
    mode: isProd ? "production" : "development",
    watch: !isProd,
    devtool: 'cheap-module-source-map',
    output: {
        path: outputDir,
        filename: "background.js"
    },
    plugins:[copyPlugin]
}