<template>
  <div class="settings-page-shortcut">
    <ui-input
      class="settings-page-shortcut__input"
      ref="revealShortcutInput"
      :model-value="revealShortcut.formattedShortcut.value"
      readonly
      placeholder="Not set"
      @keydown="revealShortcut.onKeydown"
    />
    <template v-if="revealShortcut.formattedShortcut.value">
      <ui-button-icon size="small" @click="revealShortcut.reset">
        <iui-xmark />
      </ui-button-icon>
    </template>
  </div>
</template>

<script setup lang="ts">
const revealShortcutInput = useTemplateRef('revealShortcutInput');
const revealShortcut = useRevealShortcut(revealShortcutInput);
</script>

<style scoped lang="scss">
.settings-page-shortcut {
  display: flex;
  gap: 5px;
  align-items: center;

  &__input {
    --input-placeholder__color: var(--text_accent);
  }
}
</style>
