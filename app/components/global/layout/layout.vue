<template>
  <div class="layout">
    <div class="layout__header">
      <slot name="header" />
    </div>
    <div class="layout__content">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.layout {
  display: flex;
  flex-direction: column;
  gap: 10px;
  height: 100%;

  &__content {
    flex-grow: 1;
    height: 0;
  }
}
</style>
