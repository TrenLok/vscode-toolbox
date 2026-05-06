<template>
  <span class="project-icon">
    <slot />
  </span>
</template>

<script setup lang="ts">
import type { ProjectIconProps } from '.';

const props = defineProps<ProjectIconProps>();
const iconBackground = computed(() => props.background ?? 'var(--project-icon_background)');
</script>

<style scoped lang="scss">
.project-icon {
  --project-icon__size: 35px;

  display: inline-grid;
  flex-shrink: 0;
  width: var(--project-icon__size);
  height: var(--project-icon__size);
  color: var(--text_accent);
  font-size: 14px;
  font-weight: 600;
  line-height: normal;
  background: v-bind(iconBackground);
  border-radius: var(--border_2);
  place-content: center;
}
</style>
