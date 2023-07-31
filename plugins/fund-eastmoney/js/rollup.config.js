import resolve from "@rollup/plugin-node-resolve";
import { terser } from "rollup-plugin-terser";

import commonjs from "@rollup/plugin-commonjs";
import json from '@rollup/plugin-json';

export default [
  {
    input: "./dist/inject.js",
    output: {
      dir: "./dist",
      entryFileNames: "[name].min.js",
      format: "esm",
    },
    plugins: [resolve({ browser: true }), terser({ format: { comments: false } }), commonjs({ requireReturnsDefault: "auto" }), json()],
  },
  {
    input: "./dist/crawler_lazy.js",
    output: {
      dir: "./dist",
      entryFileNames: "[name].min.js",
      format: "esm",
    },
    plugins: [resolve({ browser: true }), terser({ format: { comments: false } }), commonjs({ requireReturnsDefault: "auto" }), json()],
  },
];
