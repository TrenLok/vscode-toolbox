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
          <ui-button-primary @click="getLatestVSCodeVersion">
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

const currentVersionVSCode = ref<string | undefined>();
const latestVersionVSCode = ref<string | undefined>();

const isHasUpdate = computed(() => {
  return latestVersionVSCode.value && latestVersionVSCode.value !== currentVersionVSCode.value;
});

try {
  const version = await vscode.getVersion();
  currentVersionVSCode.value = version?.version;
} catch {
  //
}

async function getLatestVSCodeVersion() {
  try {
    const res = await fetch('https://update.code.visualstudio.com/api/releases/stable');
    const versions: string[] = await res.json();
    latestVersionVSCode.value = versions[0];
  } catch (error_) {
    useTauriLogError(`Couldn't get the latest version of vscode: ${error_}`);
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
