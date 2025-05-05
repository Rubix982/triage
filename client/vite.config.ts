import { defineConfig } from "vite";
import electron from "vite-plugin-electron";

export default defineConfig({
  root: ".",
  base: "./",
  plugins: [
    electron([
      {
        // Main process entry
        entry: "electron/main.ts",
      },
    ]),
  ],
  build: {
    outDir: "dist",
  },
});
