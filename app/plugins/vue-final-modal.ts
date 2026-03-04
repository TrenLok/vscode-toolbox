import { createVfm } from 'vue-final-modal';

export default defineNuxtPlugin((nuxtApp) => {
  // eslint-disable-next-line ts/no-explicit-any
  const vfm = createVfm() as any;

  nuxtApp.vueApp.use(vfm);
});
