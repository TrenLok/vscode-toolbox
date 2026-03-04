<template>
  <ui-button-or-anchor
    :class="classNames({ ...props })"
    :is-disabled="isDisabled"
    :href="href"
    @click="emit('click', $event)"
  >
    <span class="button-primary__holder">
      <template v-if="$slots.icon">
        <span class="button-primary__icon">
          <slot name="icon" />
        </span>
      </template>
      <template v-if="$slots.default">
        <span class="button-primary__text">
          <slot />
        </span>
      </template>
    </span>
  </ui-button-or-anchor>
</template>

<script setup lang="ts">
import type { ButtonPrimaryProps } from '.';
import type { ButtonOrAnchorEmits } from '../or-anchor';

const props = withDefaults(defineProps<ButtonPrimaryProps>(), {
  size: 'default',
  width: 'default',
  color: 'default',
  isDisabled: false,
  href: undefined,
});
const emit = defineEmits<ButtonOrAnchorEmits>();

const classNames = bmc<ButtonPrimaryProps>('button-primary', {
  modifiers: {
    isDisabled: {
      modifier: 'state',
      stateIfTrue: 'disabled',
    },
  },
  whitelist: ['size', 'width', 'color', 'isDisabled'],
});
</script>

<style scoped lang="scss">
.button-primary {
  position: relative;
  display: inline-block;
  padding: var(--button-primary__padding);
  color: var(--text_accent);
  font-size: var(--button-primary__font-size);
  font-weight: 500;
  line-height: normal;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  user-select: none;
  background-color: var(--button-primary__background);
  border-radius: var(--button-primary__border-radius);
  transition: .3s ease-out;
  transition-property: color, background-color;

  &:hover,
  &:focus-visible {
    &:not(.button-primary_state_disabled) {
      --button-primary__background: var(--button-primary__background_hover);
    }
  }

  &:active {
    &:not(.button-primary_state_disabled) {
      --button-primary__background: var(--button-primary__background_active);
    }
  }

  &__holder {
    display: flex;
    gap: 3px;
    align-items: center;
    justify-content: center;
    width: 100%;
  }

  &__icon {
    font-size: var(--button-primary_icon__font-size);
  }

  &__text {
    text-align: center;
  }

  &_size {
    &_default {
      --button-primary__padding: 5px 12px;
      --button-primary__font-size: 14px;
      --button-primary_icon__font-size: 12px;
      --button-primary__border-radius: var(--border_1);
    }

    &_large {
      --button-primary__padding: 12px 22px;
      --button-primary__font-size: 16px;
      --button-primary_icon__font-size: 14px;
      --button-primary__border-radius: var(--border_0);
    }
  }

  &_color {
    &_default {
      --button-primary__background: var(--primary);
      --button-primary__background_hover: var(--primary_hover);
      --button-primary__background_active: var(--primary_active);
    }

    &_gray {
      --button-primary__background: var(--gray_base_2);
      --button-primary__background_hover: var(--gray_base_4);
      --button-primary__background_active: var(--gray_base_5);
    }
  }

  &_state {
    &_disabled {
      opacity: .6;
    }
  }

  &_width {
    &_full {
      width: 100%;
    }
  }
}
</style>
