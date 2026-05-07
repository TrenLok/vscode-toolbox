<template>
  <ui-version-block>
    <template #logo>
      <picture>
        <img
          src="~assets/images/vscode.png"
          alt=""
        >
      </picture>
    </template>
    <template #name>
      VSCode
    </template>
    <template #versions>
      <template v-if="currentVersionVSCode">
        <ui-version>
          {{ currentVersionVSCode }}
        </ui-version>
      </template>
      <template v-else>
        <ui-version type="error">
          <template #title>
            VSCode is not installed
          </template>
        </ui-version>
      </template>
      <template v-if="isHasUpdate">
        <ui-version>
          <template #title>
            A new version is available:
          </template>
          <template #default>
            {{ latestVersionVSCode }}
          </template>
        </ui-version>
      </template>
      <template v-if="latestVersionVSCode && !isHasUpdate">
        <ui-version>
          <template #title>
            No updates available
          </template>
        </ui-version>
      </template>
    </template>
    <template #action>
      <template v-if="!currentVersionVSCode">
        <ui-button-primary @click="openVsCodeDownloadPage">
          Install VSCode
        </ui-button-primary>
      </template>
      <template v-else>
        <template v-if="isHasUpdate">
          <ui-button-primary @click="openVsCodeDownloadPage">
            Install update
          </ui-button-primary>
        </template>
        <template v-else>
          <ui-button-primary
            :is-disabled="isUpdateCheck"
            :is-loading="isUpdateCheck"
            @click="getLatestVSCodeVersion(true)"
          >
            Check for updates
          </ui-button-primary>
        </template>
      </template>
    </template>
  </ui-version-block>
</template>

<script setup lang="ts">
const vscode = useVscode();
const appSettings = useAppSettings();
const { notify } = useNotification();

const currentVersionVSCode = ref<string | undefined>();
const latestVersionVSCode = ref<string | undefined>();
const isUpdateCheck = ref(false);

const isHasUpdate = computed(() => {
  return latestVersionVSCode.value && latestVersionVSCode.value !== currentVersionVSCode.value;
});

try {
  const version = await vscode.getVersion();
  currentVersionVSCode.value = version?.version;
} catch {
  //
}

async function getLatestVSCodeVersion(shouldNotify: boolean = false) {
  isUpdateCheck.value = true;

  try {
    await withMinDuration(async () => {
      const res = await fetch('https://update.code.visualstudio.com/api/releases/stable');
      const versions: string[] = await res.json();
      latestVersionVSCode.value = versions[0];
    });
  } catch (error_) {
    if (shouldNotify) {
      notify({
        text: 'Failed to check for updates',
        type: 'error',
      });
    }
    useTauriLogError(`Couldn't get the latest version of vscode: ${error_}`);
  } finally {
    isUpdateCheck.value = false;
  }
}

async function openVsCodeDownloadPage() {
  await useTauriOpenerOpenUrl('https://code.visualstudio.com');
}

onMounted(async () => {
  if (!appSettings.autoCheckUpdates.value) return;
  await getLatestVSCodeVersion();
});
</script>

<style scoped lang="scss">

</style>
