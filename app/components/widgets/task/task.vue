<template>
  <ui-task class="task-widget">
    <div class="task-widget__content">
      <template v-if="taskStore.state.type === 'loading'">
        <ui-progress-spinner class="task-widget__spinner" />
      </template>
      {{ taskStore.state.text }}
    </div>
  </ui-task>
</template>

<script setup lang="ts">
const taskStore = useTaskStore();
</script>

<style scoped lang="scss">
.task-widget {
  z-index: 10;

  &__content {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  &__spinner {
    font-size: 16px;
  }
}
</style>
