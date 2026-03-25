<template>
  <ui-button-or-anchor
    :class="classNames({ ...props })"
    :is-disabled="isDisabled"
    :href="href"
    @click="emit('click', $event)"
  >
    <slot />
  </ui-button-or-anchor>
</template>

<script setup lang="ts">
import type { ButtonIconProps } from '.';
import type { ButtonOrAnchorEmits } from '../or-anchor';

const props = withDefaults(defineProps<ButtonIconProps>(), {
  size: 'default',
});
const emit = defineEmits<ButtonOrAnchorEmits>();

const classNames = bmc<ButtonIconProps>('button-icon', {
  modifiers: {
    isDisabled: {
      modifier: 'state',
      stateIfTrue: 'disabled',
    },
  },
  whitelist: ['size', 'isDisabled'],
});
</script>

<style scoped lang="scss">
.button-icon {
  --button-icon__background: transparent;

  position: relative;
  display: grid;
  place-content: center;
  width: var(--button-icon__size);
  height: var(--button-icon__size);
  padding: 0;
  color: var(--text_accent);
  font-size: var(--button-icon__font-size);
  font-weight: 500;
  line-height: normal;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  user-select: none;
  background-color: var(--button-icon__background);
  border-radius: var(--button-icon__border-radius);
  transition: .3s ease-out;
  transition-property: color, background-color;

  &:hover,
  &:focus-visible {
    &:not(.button-icon_state_disabled) {
      --button-icon__background: var(--button-icon_background_hover);
    }
  }

  &:active {
    &:not(.button-icon_state_disabled) {
      --button-icon__background: var(--button-icon_background_active);
    }
  }

  &_size {
    &_default {
      --button-icon__size: 35px;
      --button-icon__font-size: 16px;
      --button-icon__border-radius: var(--border_1);
    }

    &_small {
      --button-icon__size: 25px;
      --button-icon__font-size: 12px;
      --button-icon__border-radius: var(--border_0);
    }

    &_tiny {
      --button-icon__size: 20px;
      --button-icon__font-size: 10px;
      --button-icon__border-radius: var(--border_0);
    }
  }

  &_state {
    &_disabled {
      opacity: .6;
    }
  }
}
</style>
