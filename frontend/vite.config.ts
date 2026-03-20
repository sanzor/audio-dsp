import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from "path"
// https://vite.dev/config/
export default defineConfig({
   server: {
    port: 3000, // 👈 your FE runs on http://localhost:3000
    host: true,
    proxy: {
      "/auth": "http://localhost:3080",
      "/v1": "http://localhost:3080",
    },
  },
  plugins: [react(),tailwindcss(),],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
   
  },
})
