<template>
  <div class="dropdown-container">
    <template v-if="$slots.search || $slots.actions">
      <div class="dropdown-container__actions">
        <template v-if="$slots.search">
          <div class="dropdown-container__search">
            <slot name="search" />
          </div>
        </template>
        <slot name="actions" />
      </div>
    </template>
    <div class="dropdown-container__content">
      <div class="dropdown-container__options">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style lang="scss">
.dropdown-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-width: 100%;
  max-height: 320px;
  padding: 5px;
  background: var(--gray_base_2);
  border-radius: var(--border_2);

  &__actions {
    display: flex;
    flex-shrink: 0;
    gap: 8px;
    width: 100%;
  }

  &__search {
    flex-grow: 1;
    flex-shrink: 0;
  }

  &__content {
    display: flex;
    overflow: hidden auto;
    flex-shrink: 1;
    flex-direction: column;
    min-height: 0;
  }

  &__options {
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    gap: 5px;
  }

  &_scroll {
    &_y {
      --scroll-width: 10px;
    }
  }
}
</style>
