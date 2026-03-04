<template>
  <b-modal class="modal-not-found">
    <b-modal-container-default class="modal-not-found__container">
      <template #title>
        Hide project
      </template>
      <template #default>
        <p class="modal-not-found__text">
          The project <b>{{ project.name }}</b> will be hidden from the list.
          You can restore it by opening it via the toolbox.
        </p>
      </template>
      <template #actions>
        <ui-button-primary @click="hideProject">
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
import type { Project } from '~/types/project';

interface Emits {
  close: [void];
}

interface Props {
  project: Project;
}

const emit = defineEmits<Emits>();
const props = defineProps<Props>();

const hiddenFolders = useHiddenFolders();

function hideProject() {
  hiddenFolders.addFolder({
    path: props.project.folder,
    isDeleted: false,
  });
  emit('close');
}
</script>

<style scoped lang="scss">

</style>
