<template>
  <header class="header">
    <div class="header__logo">
      <slot name="logo" />
    </div>
    <div class="header__actions">
      <slot name="actions" />
    </div>
  </header>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 49px;
  padding: 7px 10px;
  background-color: var(--gray_base_1);
  border-radius: var(--border_2);

  &__actions {
    display: flex;
    gap: 2px;
    align-items: center;
  }
}
</style>
