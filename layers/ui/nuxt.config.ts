import path from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDir = path.dirname(fileURLToPath(import.meta.url));

export default defineNuxtConfig({
  alias: {
    '@ui-assets': path.join(currentDir, './app/assets'),
  },
  compatibilityDate: '2025-07-15',
  components: {
    dirs: [
      { path: './components', prefix: 'Ui', extensions: ['.vue'] },
      { path: './icons', prefix: 'Iui', extensions: ['.vue'] },
    ],
  },
});
