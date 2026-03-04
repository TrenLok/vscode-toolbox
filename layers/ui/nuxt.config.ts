export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  components: {
    dirs: [
      { path: './components', prefix: 'Ui', extensions: ['.vue'] },
      { path: './icons', prefix: 'Iui', extensions: ['.vue'] },
    ],
  },
});
