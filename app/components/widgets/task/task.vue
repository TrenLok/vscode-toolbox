<template>
  <ui-task class="task-widget">
    <div class="task-widget__content">
      <template v-if="taskStore.state.type === 'loading'">
        <ui-progress-spinner />
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
  &__content {
    display: flex;
    gap: 8px;
    align-items: center;
  }
}
</style>
