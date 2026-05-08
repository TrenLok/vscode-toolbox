<template>
  <div class="project-list" :class="{ 'project-list_state_open': expanded }">
    <div class="project-list__header-wrapper">
      <button
        type="button"
        class="project-list__header"
        @click="expanded = !expanded"
      >
        <span class="project-list__icon">
          <iui-angle-down />
        </span>
        <p class="project-list__title">
          <slot name="title" />
        </p>
      </button>
    </div>
    <template v-if="expanded">
      <div class="project-list__items">
        <slot />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
const expanded = ref(true);
</script>

<style scoped lang="scss">
.project-list {
  --project-list_icon__rotate: 0deg;
  --project-list_header__background: transparent;

  display: flex;
  flex-direction: column;
  gap: 5px;
  width: 100%;

  &__header-wrapper:hover,
  &__header-wrapper:focus-within {
    --project-list_header__background: var(--project-list_background);
  }

  &__header-wrapper {
    width: 100%;
  }

  &__header {
    display: flex;
    gap: 5px;
    align-items: center;
    width: 100%;
    padding: 5px;
    cursor: pointer;
    user-select: none;
    background-color: var(--project-list_header__background);
    border-radius: var(--border_2);
    outline: none;
  }

  &__icon {
    color: var(--gray_base_8);
    font-size: 10px;
    transition: rotate .1s ease;
    rotate: var(--project-list_icon__rotate);
  }

  &__title {
    color: var(--gray_base_8);
    font-weight: 500;
  }

  &__items {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  &_state {
    &_open {
      --project-list_icon__rotate: -180deg;
    }
  }
}
</style>
