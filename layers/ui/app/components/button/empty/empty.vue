<template>
  <ui-button-or-anchor class="button-empty" @click="emit('click', $event)">
    <span class="button-empty__holder">
      <template v-if="$slots.icon">
        <span class="button-empty__icon">
          <slot name="icon" />
        </span>
      </template>
      <template v-if="$slots.default">
        <span class="button-empty__text">
          <slot />
        </span>
      </template>
    </span>
  </ui-button-or-anchor>
</template>

<script setup lang="ts">
import type { ButtonOrAnchorEmits } from '../or-anchor';

const emit = defineEmits<ButtonOrAnchorEmits>();
</script>

<style scoped lang="scss">
.button-empty {
  --button-empty__color: white;

  position: relative;
  display: inline-block;
  padding: 5px 10px;
  color: var(--button-empty__color);
  font-weight: 500;
  line-height: normal;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  user-select: none;
  background-color: transparent;
  border-radius: 8px;
  transition: .3s ease-out;
  transition-property: color, background-color;

  &:hover,
  &:focus-visible {
    --button-empty__color: rgb(202, 202, 202);
  }

  &:active {
    --button-empty__color: rgb(189, 189, 189);
  }
}
</style>
