import { defineConfig, searchForWorkspaceRoot } from 'vite'

export default defineConfig({
  server: {
    fs: {
        // Allow serving files from two levels up to the project root
        allow: ['../../'],
      },
  },
})