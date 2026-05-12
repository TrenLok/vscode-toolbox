<template>
  <ui-version-block>
    <template #logo>
      <picture>
        <img
          v-if="currentVSCodeChannel === 'vscodium'"
          src="~assets/images/codium.png"
          alt=""
        >
        <img
          v-else
          src="~assets/images/vscode.png"
          alt=""
        >
      </picture>
    </template>
    <template #name>
      {{ vscodeDisplayName }}
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
import type { VSCodeVersionChannelType } from '~/types/vscode';

const vscode = useVscode();
const appSettings = useAppSettings();
const { notify } = useNotification();

const currentVersionVSCode = ref<string | undefined>();
const currentVSCodeChannel = ref<VSCodeVersionChannelType>('stable');
const latestVersionVSCode = ref<string | undefined>();
const isUpdateCheck = ref(false);

const isHasUpdate = computed(() => {
  return latestVersionVSCode.value && latestVersionVSCode.value !== currentVersionVSCode.value;
});

const vscodeDisplayName = computed(() => {
  switch (currentVSCodeChannel.value) {
    case 'insider': {
      return 'VSCode Insiders';
    }
    case 'vscodium': {
      return 'VSCodium';
    }
    default: {
      return 'VSCode';
    }
  }
});

async function getLatestVSCodeVersion(shouldNotify: boolean = false) {
  isUpdateCheck.value = true;

  try {
    await withMinDuration(async () => {
      if (currentVSCodeChannel.value === 'vscodium') {
        const res = await fetch('https://api.github.com/repos/VSCodium/vscodium/releases/latest');
        const release: { name?: string; tag_name?: string } = await res.json();
        latestVersionVSCode.value = release.tag_name ?? release.name;

        return;
      }

      const releaseChannel = currentVSCodeChannel.value === 'insider' ? 'insider' : 'stable';
      const res = await fetch(`https://update.code.visualstudio.com/api/releases/${releaseChannel}`);
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
  try {
    const version = await vscode.getVersion();
    currentVersionVSCode.value = version?.version;
    currentVSCodeChannel.value = version?.channel ?? 'stable';
  } catch {
  //
  }

  if (!appSettings.autoCheckUpdates.value || !currentVersionVSCode.value) return;
  await getLatestVSCodeVersion();
});
</script>

<style scoped lang="scss">

</style>
