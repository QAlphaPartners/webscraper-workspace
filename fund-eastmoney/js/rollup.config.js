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
      format: "iife",
    },
    plugins: [resolve(), terser({ format: { comments: false } }), commonjs({
      // include the playwright-core module
      include: /node_modules\/.pnpm\/playwright-core/,
      // specify how to handle default exports
      requireReturnsDefault: "auto",
    }), json()],
  },
];
