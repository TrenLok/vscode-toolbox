export function useAutostart() {
  const appStore = useAppStore();

  const status = computed(() => appStore.autostart);

  async function switchAutostart() {
    const isState = await useTauriAutostartIsEnabled();

    if (isState) await disable();
    else await enable();
  }

  async function enable() {
    const isState = await useTauriAutostartIsEnabled();
    if (isState) return;

    await useTauriAutostartEnable();

    appStore.autostart = await useTauriAutostartIsEnabled();

    useTauriLogInfo('Enable autostart app');
  }

  async function disable() {
    const isState = await useTauriAutostartIsEnabled();
    if (!isState) return;

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
