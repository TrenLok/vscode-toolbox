export function useAutostart() {
  const appStore = useAppStore();

  const status = computed(() => appStore.autostart);

  async function switchAutostart() {
    const state = await useTauriAutostartIsEnabled();

    if (state) await disable();
    else await enable();
  }

  async function enable() {
    const state = await useTauriAutostartIsEnabled();
    if (state) return;

    await useTauriAutostartEnable();

    appStore.autostart = await useTauriAutostartIsEnabled();

    useTauriLogInfo('Enable autostart app');
  }

  async function disable() {
    const state = await useTauriAutostartIsEnabled();
    if (!state) return;

    await useTauriAutostartDisable();

    appStore.autostart = await useTauriAutostartIsEnabled();

    useTauriLogInfo('Disable autostart app');
  }

  async function updateState() {
    appStore.autostart = await useTauriAutostartIsEnabled();
  }

  return {
    status,
    switchAutostart,
    updateState,
    disable,
  };
}
