<template>
  <div class="version-block">
    <div class="version-block__header">
      <div class="version-block__logo">
        <slot name="logo" />
      </div>
      <p class="version-block__name">
        <slot name="name" />
      </p>
    </div>
    <div class="version-block__versions">
      <slot name="versions" />
    </div>
    <slot name="action" />
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.version-block {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
  line-height: 1;
  user-select: none;
  background-color: var(--card_background);
  border-radius: var(--border_1);

  &__header {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  &__logo {
    &:deep(img) {
      max-width: 30px;
      max-height: 30px;
    }
  }

  &__name {
    font-weight: 600;
  }

  &__versions {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    gap: 2px;
    font-size: 13px;
  }
}
</style>
