export const useAppStore = defineStore('App', () => {
  const autostart = ref(false);
  const latestVersion = ref<string | undefined>();
  const projectsScrollContainer = ref<HTMLElement | null>(null);

  function scrollToTop() {
    projectsScrollContainer.value?.scrollTo({ top: 0 });
  }

  return {
    autostart,
    latestVersion,
    projectsScrollContainer,
    scrollToTop,
  };
});
