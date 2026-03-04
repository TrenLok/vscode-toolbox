export default defineNuxtPlugin(() => {
  if (import.meta.env.PROD) {
    const handler = (e: MouseEvent) => e.preventDefault();
    globalThis.addEventListener('contextmenu', handler);
  }
});
