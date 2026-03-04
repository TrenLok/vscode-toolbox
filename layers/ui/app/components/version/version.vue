<template>
  <p :class="classNames({ ...props })">
    <span class="version__title">
      <slot name="title">
        Current version:
      </slot>
    </span>
    <span class="version__value">
      <slot />
    </span>
  </p>
</template>

<script setup lang="ts">
import type { VersionProps } from '.';

const props = withDefaults(defineProps<VersionProps>(), {
  type: 'default',
});

const classNames = bmc<VersionProps>('version', {
  whitelist: ['type'],
});
</script>

<style scoped lang="scss">
.version {
  --version_value__color: var(--gray_base_8);

  color: var(--gray_base_8);

  &__value {
    color: var(--version_value__color);
    font-weight: 600;
  }

  &_type {
    &_error {
      --version_value__color: var(--red);
    }

    &_success {
      --version_value__color: var(--green);
    }
  }
}
</style>
