<template>
  <b-modal class="modal-vscode-not-installed">
    <b-modal-container-default class="modal-vscode-not-installed__container">
      <template #title>
        VSCode is not installed
      </template>
      <template #default>
        <p class="modal-vscode-not-installed__text">
          Install <b>VSCode</b> to launch the project.
        </p>
        <p class="modal-vscode-not-installed__text">
          After installation, restart <b>VSCode Toolbox</b>.
        </p>
      </template>
      <template #actions>
        <ui-button-primary @click="openVsCodeSite">
          Download
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

async function openVsCodeSite() {
  await useTauriOpenerOpenUrl('https://code.visualstudio.com');
  emit('close');
}
</script>

<style scoped lang="scss">

</style>
