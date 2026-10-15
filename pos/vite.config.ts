import { defineConfig } from "vitest/config";

export default defineConfig({
  root: "web",
  base: "./",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    target: "es2022",
  },
  test: {
    root: ".",
    include: ["web/src/**/*.test.ts"],
  },
});
