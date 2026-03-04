<template>
  <button
    type="button"
    class="dropdown-option"
    :class="classNames({ ...props })"
    @click="onClick"
  >
    <span class="dropdown-option__text">
      <slot />
    </span>
  </button>
</template>

<script setup lang="ts">
import type { DropdownOptionEmits, DropdownOptionProps } from '.';

const props = withDefaults(defineProps<DropdownOptionProps>(), {
  isActive: false,
});

const emit = defineEmits<DropdownOptionEmits>();

const classNames = bmc<DropdownOptionProps>('dropdown-option', {
  modifiers: {
    isActive: { modifier: 'state', stateIfTrue: 'active' },
  },
  whitelist: ['isActive'],
});

function onClick(event: MouseEvent): void {
  emit('click', event);
}
</script>

<style lang="scss">
.dropdown-option {
  --dropdown-option__text-opacity: 1;
  --dropdown-option__background-color: transparent;

  display: flex;
  gap: 10px;
  align-items: center;
  justify-content: flex-start;
  width: 100%;
  padding: 7px 10px;
  color: var(--text_accent);
  line-height: normal;
  text-align: left;
  white-space: nowrap;
  cursor: pointer;
  background-color: var(--dropdown-option__background-color);
  border: none;
  border-radius: var(--border_2);
  outline: none;
  transition: background-color .1s linear;

  &:hover,
  &:focus-visible {
    &:not(.dropdown-option_state_active) {
      --dropdown-option__background-color: var(--gray_base_5);
    }
  }

  &:active {
    &:not(.dropdown-option_state_active) {
      --dropdown-option__text-opacity: .5;
      --dropdown-option__background-color: var(--gray_base_6);
    }
  }

  &__text {
    overflow: hidden;
    flex: 1 1 100%;
    text-overflow: ellipsis;
    opacity: var(--dropdown-option__text-opacity);
    transition: opacity .1s linear;
  }

  &_state {
    &_active {
      background-color: var(--primary);
    }
  }
}
</style>
