export default defineNuxtPlugin(() => {
  if (!import.meta.env.PROD) {
    return;
  }

  const handler = (e: MouseEvent) => e.preventDefault();
  addEventListener('contextmenu', handler);
});
