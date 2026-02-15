/// <reference types="vitest" />
import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/frontend/app/test/setup.ts"],
    include: ["src/frontend/app/test/**/*.test.{ts,tsx}"],
  },
});
