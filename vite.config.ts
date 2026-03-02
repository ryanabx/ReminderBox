import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { VitePWA } from 'vite-plugin-pwa';

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
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate', // automatically update the service worker
      manifest: {
        "name": "ReminderBox",
        "short_name": "ReminderBox",
        "description": "A reminders app for your browser",
        "start_url": "/ReminderBox/",
        "id": "/ReminderBox/",
        "scope": "/ReminderBox/",
        "display": "standalone",
        "background_color": "#ffffff",
        "theme_color": "#1e88e5",
        "orientation": "portrait",
        "icons": [
          {
            "src": "android-chrome-192x192.png",
            "sizes": "192x192",
            "type": "image/png"
          },
          {
            "src": "android-chrome-512x512.png",
            "sizes": "512x512",
            "type": "image/png"
          }
        ],
        screenshots: [
          {
            src: "screenshots/screenshot-wide.png",
            sizes: "1280x720",
            type: "image/png",
            label: "Main Reminder Screen",
            form_factor: "wide"  // required for desktop "richer install"
          },
          {
            src: "screenshots/screenshot-mobile.png",
            sizes: "752x1136",
            type: "image/png",
            label: "Mobile Reminder Screen"
            // form_factor omitted, default for mobile
          }
        ]
      }
    }),
  ],
  base: "/ReminderBox/",
  define: {
    __APP_COMMIT__: JSON.stringify(getGitCommit()),
  },
})