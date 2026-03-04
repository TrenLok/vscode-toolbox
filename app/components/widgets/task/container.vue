<template>
  <template v-if="taskStore.state.visible">
    <w-task />
  </template>
</template>

<script setup lang="ts">
const taskStore = useTaskStore();
</script>
