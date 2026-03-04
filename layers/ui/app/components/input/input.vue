<template>
  <label :class="classNames({ ...props, isFocused: isFocusedRef || isFocused })">
    <input
      v-bind="$attrs"
      class="input__input"
      :value="modelValue"
      :name="name"
      :type="type"
      :placeholder="placeholder"
      :required="isRequired"
      @focus="onFocus"
      @blur="onBlur"
      @input="onInput"
      @click="emit('click', $event)"
    >
    <template v-if="isClearable && modelValue">
      <button type="button" class="input__clear" @click.stop="onClear">
        <iui-xmark />
      </button>
    </template>
    <template v-else-if="$slots['icon-after']">
      <div class="input__icon">
        <slot name="icon-after" />
      </div>
    </template>
  </label>
</template>

<script setup lang="ts">
import type { InputEmits, InputProps } from '.';

const props = withDefaults(defineProps<InputProps>(), {
  name: undefined,
  placeholder: undefined,
  isRequired: false,
  isFocused: false,
  type: 'text',
  isClearable: false,
});

const emit = defineEmits<InputEmits>();

const modelValue = defineModel<string>({
  required: false,
  default: '',
});

const isFocusedRef = ref(false);

const classNames = bmc<InputProps>('input', {
  modifiers: {
    isFocused: { modifier: 'state', stateIfTrue: 'focus' },
  },
  whitelist: ['isFocused'],
});

function onFocus(): void {
  isFocusedRef.value = true;
}

function onBlur(): void {
  isFocusedRef.value = false;
}

function onInput(event: Event): void {
  const target = event.target as HTMLInputElement;
  modelValue.value = target.value;
}

function onClear(): void {
  modelValue.value = '';
}
</script>

<style lang="scss">
@use '@/assets/scss/mixins';

.input {
  --input__color: var(--text_accent);
  --input-placeholder__color: var(--gray_base_6);
  --input-icon__color: var(--gray_base_6);
  --input__padding: 5px 10px;
  --input__border-color: var(--gray_base_5);

  position: relative;
  display: inline-flex;
  gap: 5px;
  align-items: center;
  width: 100%;
  padding: var(--input__padding);
  background-color: transparent;
  border: 1px solid var(--input__border-color);
  border-radius: var(--border_1);
  transition: border .3s ease;

  &:hover,
  &:focus-visible {
    &:not(.input_state_focus) {
      --input-icon__color: var(--gray_base_8);
      --input__border-color: var(--gray_base_8);
    }
  }

  &__input {
    flex-grow: 1;
    flex-shrink: 1;
    width: 100%;
    color: var(--input__color);
    font-size: 13px;
    font-weight: 400;
    line-height: normal;
    background-color: transparent;
    border: none;
    outline: none;

    @include mixins.placeholder(var(--input-placeholder__color));

    &:-webkit-autofill,
    &:-webkit-autofill:focus {
      transition: background-color 0s 6000000s, color 0s 6000000s !important;
    }
  }

  &__clear {
    flex-shrink: 0;
    color: var(--input-icon__color);
    font-size: 13px;
    line-height: normal;
    background: transparent;
    border: none;
  }

  &__icon {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    color: var(--input-icon__color);
    font-size: 13px;
    text-align: right;
    transition: color .3s ease;
  }

  &_state {
    &_focus {
      --input-icon__color: var(--text_accent);
      --input__border-color: var(--primary);
    }
  }
}
</style>
