<template>
  <label :class="classNames({ ...props, isActive: modelValue })">
    <input
      v-bind="$attrs"
      v-model="modelValue"
      class="checkbox__input"
      type="checkbox"
      :disabled="isDisabled"
    >
    <span class="checkbox__square">
      <span class="checkbox__icon">
        <iui-check-bold />
      </span>
    </span>
    <template v-if="$slots.default">
      <span class="checkbox__label">
        <slot />
      </span>
    </template>
  </label>
</template>

<script setup lang="ts">
import type { CheckboxProps } from '.';

const props = withDefaults(defineProps<CheckboxProps>(), {
  isDisabled: false,
});

const modelValue = defineModel<boolean>({
  required: false,
  default: false,
});

const classNames = bmc<CheckboxProps>('checkbox', {
  modifiers: {
    isDisabled: { modifier: 'state', stateIfTrue: 'disabled' },
  },
  customModifiers: {
    isActive: {
      modifier: 'state',
      stateIfTrue: 'active',
    },
  },
  whitelist: ['isDisabled'],
});
</script>

<style scoped lang="scss">
.checkbox {
  --checkbox__size: 18px;
  --checkbox__size-icon: 8px;
  --checkbox__border-color: #575757;
  --checkbox__background-color: transparent;
  --checkbox-icon__color: #ffffff;
  --checkbox-icon__opacity: 1;
  --checkbox_label__color: white;
  --checkbox__transition: .2s ease;

  position: relative;
  display: inline-flex;
  flex-shrink: 0;
  gap: 10px;
  align-items: center;
  cursor: pointer;

  &:focus-visible,
  &:hover {
    --checkbox__border-color: #535353;
    --checkbox-icon__color: #ffffff;
    --checkbox_label__color: white;

    &:is(.checkbox_state_active) {
      --checkbox__border-color: #535353;
      --checkbox__background-color: #575757;
    }
  }

  &:active {
    opacity: .8;
  }

  &__input {
    position: absolute;
    display: block;
    visibility: hidden;
    font-size: inherit;
    outline: none;
  }

  &__square {
    position: relative;
    flex-shrink: 0;
    width: var(--checkbox__size);
    height: var(--checkbox__size);
    background-color: var(--checkbox__background-color);
    border: 1px solid var(--checkbox__border-color);
    border-radius: 5px;
    -webkit-tap-highlight-color: transparent;
    transition: var(--checkbox__transition);
    transition-property: border, background-color;
  }

  &__icon {
    position: absolute;
    top: 50%;
    left: 50%;
    color: var(--checkbox-icon__color);
    font-size: var(--checkbox__size-icon);
    opacity: 0;
    transition: opacity .2s ease-out;
    transform: translate(-50%, -50%);

    .checkbox__input:checked + .checkbox__square & {
      opacity: var(--checkbox-icon__opacity);
    }
  }

  &__label {
    color: var(--checkbox_label__color);
    transition: color var(--checkbox__transition);
  }

  &_state {
    &_active {
      --checkbox__background-color: #575757;
      --checkbox__border-color: #575757;
    }
  }
}
</style>
