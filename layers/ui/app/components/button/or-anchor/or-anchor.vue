<template>
  <template v-if="href">
    <a
      :href="href"
      v-bind="$attrs"
      @click="emit('click', $event)"
      @pointerenter="emit('pointerenter', $event)"
    >
      <slot />
    </a>
  </template>
  <template v-else>
    <button
      type="button"
      :disabled="isDisabled"
      v-bind="$attrs"
      @click="emit('click', $event)"
    >
      <slot />
    </button>
  </template>
</template>

<script setup lang="ts">
import type { ButtonOrAnchorEmits, ButtonOrAnchorProps } from '.';

withDefaults(defineProps<ButtonOrAnchorProps>(), {
  isDisabled: false,
});

const emit = defineEmits<ButtonOrAnchorEmits>();
</script>

<style scoped lang="scss">
</style>
