<template>
  <div class="settings-page">
    <div class="settings-page__versions">
      <w-version-toolbox />
      <w-version-vscode />
    </div>
    <ui-settings-section>
      <template #title>
        Feature settings
      </template>
      <template #default>
        <template v-if="themeOptions.length > 1">
          <ui-segment-control>
            <template #label>
              Theme:
            </template>
            <template #default>
              <ui-segment-control-button
                v-for="themeOption in themeOptions"
                :key="themeOption.value"
                :is-active="appSettings.theme.value === themeOption.value"
                @click="setTheme(themeOption.value)"
              >
                {{ themeOption.label }}
              </ui-segment-control-button>
            </template>
          </ui-segment-control>
        </template>
        <ui-segment-control>
          <template #label>
            Project icon:
          </template>
          <template #default>
            <p-settings-project-icon-button
              :is-active="appSettings.projectIconStyle.value === 'default'"
              @click="setProjectIconStyle('default')"
            >
              <ui-project-icon>
                VT
              </ui-project-icon>
            </p-settings-project-icon-button>
            <p-settings-project-icon-button
              :is-active="appSettings.projectIconStyle.value === 'gradient'"
              @click="setProjectIconStyle('gradient')"
            >
              <ui-project-icon :background="getGradientFromString('VT')">
                VT
              </ui-project-icon>
            </p-settings-project-icon-button>
          </template>
        </ui-segment-control>
        <ui-switch
          :model-value="autostart.status.value"
          @update:model-value="autostart.switchAutostart"
        >
          Launch VScode Toolbox at system startup
        </ui-switch>
        <ui-switch
          :model-value="appSettings.vsCodeSync.value"
          @update:model-value="onSwitchVsCodeSync"
        >
          Enable synchronization with VSCode
        </ui-switch>
        <ui-switch
          :model-value="appSettings.autoCheckUpdates.value"
          @update:model-value="appSettings.switchAutoCheckUpdates"
        >
          Check for updates automatically
        </ui-switch>
      </template>
    </ui-settings-section>
    <ui-settings-section>
      <template #title>
        Global shortcut to open VSCode Toolbox
      </template>
      <p-settings-shortcut />
    </ui-settings-section>
    <ui-settings-section>
      <template #title>
        Information
      </template>
      <template #default>
        <ui-settings-link @click="openHomePage">
          <template #icon>
            <iui-github />
          </template>
          <template #default>
            Home page
          </template>
        </ui-settings-link>
        <ui-settings-link @click="openIssuePage">
          <template #icon>
            <iui-bug />
          </template>
          <template #default>
            Report an issue
          </template>
        </ui-settings-link>
      </template>
      <template #footer>
        <div class="settings-page__buttons">
          <ui-button-primary width="full" @click="openDataFolder()">
            Open data folder
          </ui-button-primary>
          <ui-button-primary width="full" @click="openLogsFolder()">
            Open logs folder
          </ui-button-primary>
        </div>
        <ui-button-primary
          width="full"
          color="gray"
          @click="modals.clearData()"
        >
          Clear application data
        </ui-button-primary>
      </template>
    </ui-settings-section>
  </div>
</template>

<script setup lang="ts">
import type { AppTheme, ProjectIconStyle } from '~/types/app-settings';

definePageMeta({
  layout: 'settings',
});

const autostart = useAutostart();
const appSettings = useAppSettings();
const modals = useModals();
const { getGradientFromString } = useStringColor();
const availableThemes = await useAvailableThemes();

const {
  syncVSCodeRecent,
} = useProjectManager();

const themeLabels: Record<AppTheme, string> = {
  default: 'Default',
  liquid_glass: 'Liquid Glass',
  vibrancy: 'Vibrancy',
  mica: 'Mica',
};

const themeOptions = availableThemes.map((value) => ({
  value,
  label: themeLabels[value],
}));

async function openLogsFolder() {
  try {
    const dir = await useTauriPathAppLogDir();
    await useTauriOpenerOpenPath(dir);
  } catch (error_) {
    useTauriLogError(`Couldn't open logs folder: ${error_}`);
  }
}

async function openDataFolder() {
  try {
    const dir = await useTauriPathAppDataDir();
    await useTauriOpenerOpenPath(dir);
  } catch (error_) {
    useTauriLogError(`Couldn't open data folder: ${error_}`);
  }
}

async function openHomePage() {
  await useTauriOpenerOpenUrl('https://github.com/trenlok/vscode-toolbox');
}

async function openIssuePage() {
  await useTauriOpenerOpenUrl('https://github.com/trenlok/vscode-toolbox/issues');
}

async function onSwitchVsCodeSync() {
  await appSettings.switchVsCodeSync();

  if (appSettings.vsCodeSync.value) {
    await syncVSCodeRecent();
  }
}

async function setTheme(value: AppTheme) {
  const nextTheme = availableThemes.includes(value) ? value : 'default';
  await appSettings.switchTheme(nextTheme);
}

async function setProjectIconStyle(value: ProjectIconStyle) {
  await appSettings.switchProjectIconStyle(value);
}

await autostart.updateState();
</script>

<style scoped lang="scss">
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 10px;

  &__versions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  &__buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 5px;
  }
}
</style>
