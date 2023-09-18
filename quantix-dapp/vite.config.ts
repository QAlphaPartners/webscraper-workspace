import { defineConfig } from "vite";
import postcssNesting from 'postcss-nesting';
import postcssImport from 'postcss-import';
import autoprefixer from 'autoprefixer';
import cssnano from 'cssnano';

// https://vitejs.dev/config/
export default defineConfig({

  css: {
    postcss: {
      from: "src/pcss/main.pcss",
      to:"dist-css/main.css", 
      plugins:[
        postcssImport(),
        postcssNesting,
        autoprefixer,
        cssnano
      ]
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
});
