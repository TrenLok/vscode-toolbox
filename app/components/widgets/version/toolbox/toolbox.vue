<template>
  <ui-version-block>
    <template #logo>
      <picture>
        <img
          src="~assets/images/logo@64px.png"
          alt=""
        >
      </picture>
    </template>
    <template #name>
      VSCode Toolbox
    </template>
    <template #versions>
      <ui-version>
        {{ appVersion }}
      </ui-version>
      <template v-if="appUpdate.latestVersion.value">
        <ui-version>
          <template #title>
            A new version is available:
          </template>
          <template #default>
            {{ appUpdate.latestVersion.value }}
          </template>
        </ui-version>
      </template>
      <template v-if="appUpdate.updateIsChecked.value && !appUpdate.latestUpdate.value">
        <ui-version>
          <template #title>
            No updates available
          </template>
        </ui-version>
      </template>
    </template>
    <template #action>
      <template v-if="!appUpdate.latestUpdate.value">
        <ui-button-primary
          :is-loading="appUpdate.updateCheckPending.value"
          :is-disabled="appUpdate.updateCheckPending.value"
          @click="appUpdate.checkUpdates(true)"
        >
          Check for updates
        </ui-button-primary>
      </template>
      <template v-else>
        <ui-button-primary
          :is-loading="downloadInProgress"
          :is-disabled="downloadInProgress"
          @click="installUpdate()"
        >
          Install update
        </ui-button-primary>
      </template>
    </template>
  </ui-version-block>
</template>

<script setup lang="ts">
const appVersion = await useTauriAppGetVersion();
const appUpdate = useAppUpdate();
const appSettings = useAppSettings();

const taskStore = useTaskStore();
const downloadInProgress = ref(false);

async function installUpdate() {
  try {
    const update = await useTauriUpdaterCheck();

    if (update) {
      const percent = ref(0);
      const downloaded = ref(0);
      const contentLength = ref<number>(0);

      downloadInProgress.value = true;

      taskStore.show({
        text: `Downloading update ${percent.value}%`,
        type: 'loading',
      });

      await update.download((event) => {
        switch (event.event) {
          case 'Started': {
            contentLength.value = event.data.contentLength ?? 0;
            downloaded.value = 0;
            useTauriLogInfo(`Started downloading ${event.data.contentLength} bytes`);
            break;
          }
          case 'Progress': {
            downloaded.value += event.data.chunkLength;
            taskStore.update({
              text: `Downloading update ${calculatePercentage(downloaded.value, contentLength.value)}%`,
            });
            break;
          }
          case 'Finished': {
            useTauriLogInfo('Download finished');
            taskStore.update({ text: 'Installing update' });
            break;
          }
        }
      });

      await update.install();

      downloadInProgress.value = false;
      taskStore.hide();

      useTauriLogInfo('Update installed');
      await useTauriProcessRelaunch();
    }
  } catch (error_) {
    downloadInProgress.value = false;
    taskStore.hide();
    useTauriLogError(`Update error: ${error_}`);
  }
}

onMounted(async () => {
  if (!appSettings.autoCheckUpdates.value) return;
  await appUpdate.checkUpdates(true);
});
</script>

<style scoped lang="scss">

</style>
