import resolve from "@rollup/plugin-node-resolve";

import commonjs from "@rollup/plugin-commonjs";
import json from '@rollup/plugin-json';

export default [
  {
    input: "./dist-jslib/jsA.js",
    output: {
      dir: "./dist-jslib",
      entryFileNames: "[name].min.js",
      format: "esm",
    },
    plugins: [resolve({ browser: true }),  commonjs({ requireReturnsDefault: "auto" }), json()],
  },
  {
    input: "./dist-jslib/finance-yahoo/crawler.js",
    output: {
      dir: "./dist-jslib",
      entryFileNames: "finance-yahoo-[name].min.js",
      format: "esm",
    },
    plugins: [resolve({ browser: true }),  commonjs({ requireReturnsDefault: "auto" }), json()],
  },
];
