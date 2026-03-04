import { rhapsodic } from '@rhapsodic/eslint-config';

export default rhapsodic({
  vue: {
    a11y: true,
  },
  typescript: true,
  ignores: [
    'src-tauri/',
    'app/modules/tauri.ts', // TODO: figure out how to remove this file from tsconfig exclude
  ],
}, [
  {
    rules: {
      'unicorn/no-array-sort': 'off',
    },
  },
]);
