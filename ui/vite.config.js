import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig(() => {
  let serverEndpoint = 'http://127.0.0.1:4698'
  if (import.meta.env['VITE_INDEXER_ENDPOINT'] !== undefined) {
    serverEndpoint = import.meta.env.VITE_INDEXER_ENDPOINT
  }
  return {
    plugins: [vue()],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
    server: {
      proxy: {
        '/indexer/': {
          target: serverEndpoint,
          rewrite: (path) => path.replace(/^\/indexer/, ''),
        },
      },
      host: '127.0.0.1',
    },
  }
})
