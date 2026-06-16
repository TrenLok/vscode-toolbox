export default defineNuxtConfig({
  modules: [
    '@vueuse/nuxt',
    '@pinia/nuxt',
    'nuxt3-notifications',
  ],
  app: {
    head: {
      htmlAttrs: {
        class: 'page',
      },
      bodyAttrs: {
        class: 'page__body',
      },
      title: 'VSCode Toolbox',
      charset: 'utf8',
      viewport: 'width=device-width, initial-scale=1',
      meta: [
        { name: 'format-detection', content: 'no' },
      ],
    },
  },
  extends: [
    './layers/ui',
  ],
  css: [
    '@/assets/scss/optimize.scss',
    '@/assets/scss/typography.scss',
    '@/assets/scss/theme.scss',
    '@/assets/scss/fonts.scss',
    '@/assets/scss/global.scss',
    '@/assets/scss/modal.scss',
    'vue-final-modal/style.css',
  ],
  components: {
    dirs: [
      { path: '@/components/global', prefix: 'B', extensions: ['.vue'] },
      { path: '@/components/pages', prefix: 'P', extensions: ['.vue'] },
      { path: '@/components/widgets', prefix: 'W', extensions: ['.vue'] },
      { path: '@/components/modals', prefix: 'M', extensions: ['.vue'] },
    ],
  },
  ssr: false,
  dir: {
    modules: 'app/modules',
  },
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      hmr: {
        protocol: 'ws',
        host: '0.0.0.0',
        port: 3001,
      },
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
    optimizeDeps: {
      include: [
        '@rhapsodic/bem-classnames',
        '@tauri-apps/api/app',
        '@tauri-apps/api/core',
        '@tauri-apps/api/path',
        '@tauri-apps/api/webviewWindow',
        '@tauri-apps/api/window',
        '@tauri-apps/plugin-autostart',
        '@tauri-apps/plugin-dialog',
        '@tauri-apps/plugin-fs',
        '@tauri-apps/plugin-log',
        '@tauri-apps/plugin-notification',
        '@tauri-apps/plugin-opener',
        '@tauri-apps/plugin-os',
        '@tauri-apps/plugin-process',
        '@tauri-apps/plugin-store',
        '@tauri-apps/plugin-updater',
        'perfect-scrollbar',
        'vue-final-modal',
      ],
    },
  },
  devServer: {
    host: '0.0.0.0',
    port: 3050,
  },
  router: {
    options: {
      scrollBehaviorType: 'smooth',
    },
  },
  devtools: {
    enabled: true,
  },
  experimental: {
    typedPages: true,
  },
  typescript: {
    typeCheck: true,
    tsConfig: {
      include: [
        '../eslint.config.ts',
        '../bump.config.ts',
      ],
    },
  },
  compatibilityDate: '2025-09-01',
});
