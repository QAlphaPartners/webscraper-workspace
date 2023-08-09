import resolve from "@rollup/plugin-node-resolve";

import commonjs from "@rollup/plugin-commonjs";
import json from '@rollup/plugin-json';

export default [
  {
    input: "./dist-jslib/index.js",
    output: {
      dir: "./dist-jslib",
      entryFileNames: "[name].min.js",
      format: "esm",
    },
    plugins: [resolve({ browser: true }),  commonjs({ requireReturnsDefault: "auto" }), json()],
  },
  
];
