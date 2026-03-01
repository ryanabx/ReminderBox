import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

import { execSync } from "child_process"

function getGitCommit() {
  try {
    return execSync("git rev-parse --short HEAD").toString().trim()
  } catch {
    return "unknown"
  }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    __APP_COMMIT__: JSON.stringify(getGitCommit()),
  },
})