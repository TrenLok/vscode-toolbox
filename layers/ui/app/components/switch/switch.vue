<template>
  <label :class="classNames({ ...props, isActive: modelValue })">
    <span class="switch__wrapper">
      <input
        v-model="modelValue"
        :disabled="props.isDisabled"
        :aria-readonly="props.isReadonly || undefined"
        @click="onInputClick"
        type="checkbox"
        class="switch__input"
      >
      <span class="switch__handle" />
    </span>
    <template v-if="$slots.default">
      <span class="switch__text">
        <slot />
      </span>
    </template>
  </label>
</template>

<script setup lang="ts">
import type { SwitchProps } from '.';

const modelValue = defineModel<boolean>({
  default: false,
});

const props = withDefaults(defineProps<SwitchProps>(), {
  isDisabled: false,
  isReadonly: false,
});

const classNames = bmc<SwitchProps & { isActive: boolean }>('switch', {
  modifiers: {
    isActive: {
      modifier: 'state',
      stateIfTrue: 'active',
    },
    isDisabled: {
      modifier: 'state',
      stateIfTrue: 'disabled',
    },
    isReadonly: {
      modifier: 'state',
      stateIfTrue: 'readonly',
    },
  },
  whitelist: ['isActive', 'isDisabled', 'isReadonly'],
});

function onInputClick(event: MouseEvent): void {
  if (props.isReadonly) {
    event.preventDefault();
  }
}
</script>

<style scoped lang="scss">
.switch {
  --switch__track-background-color: var(--gray_base_3);
  --switch__handle-background-color: var(--gray_base_1);
  --switch__handle-opacity: 1;
  --switch__active-background-color: var(--primary);
  --switch__active-background-hover: var(--gray_base_4);
  --switch__active-background-active: var(--gray_base_5);
  --switch__width: 50px;
  --switch__height: 25px;
  --switch__padding: 4px;
  --switch__handle-offset: 8px;

  display: inline-flex;
  gap: 5px;
  align-items: center;

  &:hover,
  &:focus-visible {
    &:not(.switch_state_active) {
      --switch__track-background-color: var(--gray_base_4);
    }
  }

  &:active {
    &:not(.switch_state_active) {
      --switch__track-background-color: var(--gray_base_5);
    }
  }

  &__wrapper {
    position: relative;
    display: block;
    flex-shrink: 0;
    width: var(--switch__width);
    height: var(--switch__height);
    padding: var(--switch__padding);
    cursor: pointer;
    user-select: none;
    background-color: var(--switch__track-background-color);
    border: 0;
    border-radius: 60px;
    transition: .2s ease;
    transition-property: background-color, border-color;
  }

  &__input {
    display: none;
  }

  &__handle {
    display: block;
    height: 100%;
    aspect-ratio: 1/1;
    background-color: var(--switch__handle-background-color);
    border-radius: 50%;
    opacity: var(--switch__handle-opacity);
    transition: .2s ease;
    transition-property: background-color, transform, opacity;
  }

  &_state {
    &_active {
      --switch__track-background-color: var(--switch__active-background-color);

      &:active {
        --switch__track-background-color: var(--primary_active);
      }

      &:hover,
      &:focus-visible {
        --switch__track-background-color: var(--primary_hover);
      }

      .switch__handle {
        transform: translateX(calc(100% + var(--switch__handle-offset)));
      }
    }

    &_disabled {
      cursor: not-allowed;
      pointer-events: none;
      opacity: .5;
    }

    &_readonly {
      cursor: default;
    }
  }
}
</style>
