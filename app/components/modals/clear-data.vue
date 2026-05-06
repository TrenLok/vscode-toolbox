<template>
  <b-modal class="modal-clear-data">
    <b-modal-container-default class="modal-clear-data__container">
      <template #title>
        Clearing data
      </template>
      <template #default>
        <div class="modal-clear-data__content">
          <p>
            Selected app data will be deleted.
          </p>
          <ui-switch v-model="resetProjects">
            Reset projects
          </ui-switch>
          <ui-switch v-model="resetHiddenProjects">
            Reset hidden projects
          </ui-switch>
          <ui-switch v-model="resetSettings">
            Reset settings
          </ui-switch>
        </div>
      </template>
      <template #actions>
        <ui-button-primary @click="confirm">
          Confirm
        </ui-button-primary>
        <ui-button-empty @click="emit('close')">
          Cancel
        </ui-button-empty>
      </template>
    </b-modal-container-default>
  </b-modal>
</template>

<script setup lang="ts">
interface Emits {
  close: [void];
}

const emit = defineEmits<Emits>();

const projectManager = useProjectManager();
const appSettings = useAppSettings();
const hiddenFolders = useHiddenFolders();
const autostart = useAutostart();

const resetProjects = ref(true);
const resetHiddenProjects = ref(true);
const resetSettings = ref(false);

async function confirm() {
  await useTauriLogInfo('Clear app data');

  if (resetProjects.value) {
    await projectManager.clearDb();
  }

  if (resetHiddenProjects.value) {
    await hiddenFolders.clearDb();
  }

  if (resetSettings.value) {
    await appSettings.clearDb();
    await autostart.disable();
  }

  if (appSettings.vsCodeSync.value) {
    await projectManager.syncVSCodeRecent();
  }

  emit('close');
}
</script>

<style scoped lang="scss">
.modal-clear-data {
  &__container {
    --modal-container__width: 400px;
  }

  &__content {
    display: grid;
    gap: 10px;
  }
}
</style>
