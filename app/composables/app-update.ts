export function useAppUpdate() {
  const appUpdateStore = useAppUpdateStore();

  const latestUpdate = computed(() => appUpdateStore.latestUpdate);
  const latestVersion = computed(() => latestUpdate.value?.version);
  const updateIsChecked = computed(() => appUpdateStore.updateIsChecked);
  const isCheckUpdate = ref(false);

  async function checkUpdates() {
    isCheckUpdate.value = true;

    try {
      await withMinDuration(async () => {
        appUpdateStore.latestUpdate = await useTauriUpdaterCheck();

        if (appUpdateStore.latestUpdate) {
          useTauriLogInfo(`Found update ${appUpdateStore.latestUpdate.version} from ${appUpdateStore.latestUpdate.date}`);
        }

        appUpdateStore.updateIsChecked = true;
      });
    } catch (error) {
      useTauriLogError(`Couldn't check for updates: ${error}`);
    } finally {
      isCheckUpdate.value = false;
    }
  }

  return {
    latestUpdate,
    latestVersion,
    updateIsChecked,
    isCheckUpdate,
    checkUpdates,
  };
}
