<template>
  <b-modal class="modal-not-found">
    <b-modal-container-default class="modal-not-found__container">
      <template #title>
        Project not found
      </template>
      <template #default>
        <p class="modal-not-found__text">
          The path <b>{{ normalizedFolder }}</b> does not exist. <br>
          This directory may have been moved or deleted.
        </p>
      </template>
      <template #actions>
        <ui-button-primary @click="hideProject">
          Remove from list
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

interface Props {
  folder: string;
}

const emit = defineEmits<Emits>();
const props = defineProps<Props>();

const hiddenFolders = useHiddenFolders();

const normalizedFolder = computed(() => normalizeWindowsPathDrive(props.folder, 'upper'));

function hideProject() {
  hiddenFolders.addFolder({
    path: props.folder,
    isDeleted: true,
  });
  emit('close');
}
</script>

<style scoped lang="scss">

</style>
