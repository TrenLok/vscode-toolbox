<template>
  <span class="progress-spinner" />
</template>

<script setup lang="ts">
</script>

<style scoped lang="scss">
@keyframes spin {
  0% { transform: rotate(0deg); }

  100% { transform: rotate(360deg); }
}

.progress-spinner {
  --progress-spinner__size: 16px;
  --progress-spinner__border-size: 2px;

  position: relative;
  display: block;
  width: var(--progress-spinner__size);
  height: var(--progress-spinner__size);
  border: var(--progress-spinner__border-size) solid var(--gray_base_2);
  border-top: var(--progress-spinner__border-size) solid var(--gray_base_8);
  border-radius: 50%;
  animation: spin 1s infinite linear;
}
</style>
