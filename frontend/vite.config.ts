/// <reference types="vitest/config" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from "path"
// https://vite.dev/config/
export default defineConfig({
  test: {
    // Pure-function/pipeline unit tests (e.g. GraphCompiler.ts) run under
    // plain Node — no DOM/jsdom needed. Reuses this file's own `@` alias so
    // tests import with the same "@/..." convention as app code.
    environment: 'node',
    include: ['src/**/*.test.ts', 'src/**/*.test.tsx'],
  },
   server: {
    port: 3000, // 👈 your FE runs on http://localhost:3000
    host: true,
    proxy: {
      '/api': {
        target: 'http://localhost:3080',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  plugins: [react(),tailwindcss(),],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },

  },
})
