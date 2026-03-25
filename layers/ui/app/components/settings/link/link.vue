<template>
  <ui-button-or-anchor
    class="settings-link"
    :href="href"
    @click="emit('click', $event)"
  >
    <span class="settings-link__icon">
      <slot name="icon" />
    </span>
    <span class="settings-link__text">
      <slot />
    </span>
  </ui-button-or-anchor>
</template>

<script setup lang="ts">
import type { ButtonOrAnchorEmits, ButtonOrAnchorProps } from '../../button/or-anchor';

withDefaults(defineProps<ButtonOrAnchorProps>(), {
  href: undefined,
  isDisabled: false,
});

const emit = defineEmits<ButtonOrAnchorEmits>();
</script>

<style scoped lang="scss">
.settings-link {
  --settings-link_text__color: var(--text_primary);

  display: inline-flex;
  gap: 5px;
  align-items: center;

  &:hover,
  &:focus-visible {
    --settings-link_text__color: var(--text_primary_hover);
  }

  &:active {
    --settings-link_text__color: var(--text_primary_active);
  }

  &__icon {
    width: 18px;
    font-size: 16px;
  }

  &__text {
    color: var(--settings-link_text__color);
  }
}
</style>
