<template>
  <rui-input
    v-bind="props"
    v-model="stringModelValue"
    type="number"
  >
    <template v-for="(slot, index) of Object.keys($slots)" :key="index" #[slot]>
      <slot :name="slot" />
    </template>
  </rui-input>
</template>

<script setup lang="ts">
import type { InputNumberProps } from '.';

const props = defineProps<InputNumberProps>();

const modelValue = defineModel<number | null>({
  required: false,
  default: null,
});

const defaultValue = computed(() => {
  if (props.min != null) {
    return +props.min;
  }

  return 0;
});

const stringModelValue = computed<string>({
  get: () => modelValue.value?.toString() ?? '',
  set: (value) => {
    if (value.trim() === '' || Number.isNaN(+value)) {
      if (props.isRequired) {
        modelValue.value = defaultValue.value;
      } else {
        modelValue.value = null;
      }
    } else {
      modelValue.value = +value;
    }
  },
});
</script>

<style lang="scss">

</style>
