<template>
  <ui-button-or-anchor
    :class="classNames({ ...props })"
    @click="emit('click', $event)"
  >
    <span class="segment-control-button__holder">
      <slot />
    </span>
  </ui-button-or-anchor>
</template>

<script setup lang="ts">
import type { SegmentControlButtonProps } from '.';
import type { ButtonOrAnchorEmits } from '../../button/or-anchor';

const props = withDefaults(defineProps<SegmentControlButtonProps>(), {
  isActive: false,
});
const emit = defineEmits<ButtonOrAnchorEmits>();

const classNames = bmc<SegmentControlButtonProps>('segment-control-button', {
  modifiers: {
    isActive: {
      modifier: 'state',
      stateIfTrue: 'active',
    },
  },
  whitelist: ['isActive'],
});
</script>

<style scoped lang="scss">
.segment-control-button {
  --segment-control-button__border-color: var(--gray_base_5);

  display: inline-block;
  padding: 3px 10px;
  color: var(--button-empty__color);
  font-weight: 500;
  line-height: normal;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  user-select: none;
  background-color: transparent;
  border: 1px solid var(--segment-control-button__border-color);
  transition: .3s ease-out;
  transition-property: border-color, background-color;

  &:only-child {
    border-radius: var(--border_1);
  }

  &:first-child {
    &:not(.segment-control-button:only-child) {
      border-right: none;
      border-radius: var(--border_1) 0 0 var(--border_1);
    }
  }

  &:last-child {
    &:not(.segment-control-button:only-child) {
      border-left: none;
      border-radius: 0 var(--border_1) var(--border_1) 0;
    }
  }

  &:hover,
  &:focus-visible {
    &:not(.segment-control-button_state_active) {
      background-color: var(--segment-control_button_hover);
    }
  }

  &:active {
    &:not(.segment-control-button_state_active) {
      background-color: var(--segment-control_button_active);
    }
  }

  &_state {
    &_active {
      --segment-control-button__border-color: var(--primary);

      background-color: var(--primary);
    }
  }
}
</style>
