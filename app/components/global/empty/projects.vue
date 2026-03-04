<template>
  <div class="empty-projects">
    <p>You don't have any open projects yet.</p>
    <ui-button-primary @click="emit('openProject')">
      Open a new project
    </ui-button-primary>
    <p>
      You can also <nuxt-link :to="{ name: 'settings' }">enable synchronization</nuxt-link><br>
      of recent projects with VSCode.
    </p>
  </div>
</template>

<script setup lang="ts">
interface Emits {
  openProject: [void];
}

const emit = defineEmits<Emits>();
</script>

<style scoped lang="scss">
.empty-projects {
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: center;
  width: 100%;
  text-align: center;
}
</style>
