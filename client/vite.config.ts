import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import electron from 'vite-plugin-electron'
import electronRenderer from 'vite-plugin-electron-renderer'

export default defineConfig({
  root: 'src/',
  plugins: [
    react(),
    electron([
      {
        // Main process entry
        entry: 'main.ts',
      },
    ]),
    electronRenderer(),
  ],
  optimizeDeps: {
    exclude: ['@mapbox/node-pre-gyp', '@mswjs/interceptors', 'nock'],
  },
})
