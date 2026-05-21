<template>
  <button type="button" :class="classNames({ ...props })">
    <slot />
  </button>
</template>

<script setup lang="ts">
interface Props {
  isActive?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isActive: false,
});

const classNames = bmc<Props>('settings-project-icon-button', {
  isActive: flag('state', 'active'),
});
</script>

<style scoped lang="scss">
.settings-project-icon-button {
  --settings-project-icon-button__border-color: transparent;

  margin-right: 3px;
  padding: 2px;
  border: 1px solid var(--settings-project-icon-button__border-color);
  border-radius: 11px;
  outline: none;
  transition: border-color .3s ease;

  &:hover,
  &:focus-visible {
    --settings-project-icon-button__border-color: var(--primary_hover);
  }

  &:active {
    --settings-project-icon-button__border-color: var(--primary_active);
  }

  &_state {
    &_active {
      --settings-project-icon-button__border-color: var(--primary);
    }
  }
}
</style>
