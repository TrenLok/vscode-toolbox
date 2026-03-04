<template>
  <ui-dropdown
    content-position-x="right"
    content-position-y="bottom"
    ref="project-dropdown"
  >
    <template #button="{ toggle }">
      <ui-button-icon
        size="small"
        @click="toggle"
      >
        <iui-ellipsis-vertical />
      </ui-button-icon>
    </template>
    <template #content="{ close }">
      <ui-dropdown-container>
        <ui-dropdown-option @click="close(); emit('hide')">
          Hide project from list
        </ui-dropdown-option>
        <ui-dropdown-option @click="close(); emit('openFolder')">
          Show in Explorer
        </ui-dropdown-option>
      </ui-dropdown-container>
    </template>
  </ui-dropdown>
</template>

<script setup lang="ts">
interface Emits {
  hide: [void];
  openFolder: [void];
}

const emit = defineEmits<Emits>();

const bus = useEventBus('blur-window');

const dropdown = useTemplateRef('project-dropdown');

const stop = bus.on(() => {
  dropdown.value?.close();
});

onUnmounted(() => {
  stop();
});
</script>

<style scoped lang="scss">

</style>
