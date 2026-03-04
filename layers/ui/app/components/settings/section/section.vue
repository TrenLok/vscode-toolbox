<template>
  <div class="settings-section">
    <p class="settings-section__title">
      <slot name="title" />
    </p>
    <div class="settings-section__body">
      <slot />
    </div>
    <template v-if="$slots.footer">
      <div class="settings-section__footer">
        <slot name="footer" />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.settings-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
  background-color: var(--gray_base_1);
  border-radius: var(--border_2);

  &__title {
    font-weight: 600;
  }

  &__body,
  &__footer {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
}
</style>
