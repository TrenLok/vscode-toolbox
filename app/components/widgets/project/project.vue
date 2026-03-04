<template>
  <ui-project class="w-project" :inactive="inactive" @click="emit('open')">
    <template #icon>
      {{ getTwoLettersFromDirectoryName(project.name) }}
    </template>
    <template #title>
      {{ project.name }}
    </template>
    <template #subtitle>
      <template v-if="!isLaunching">
        {{ normalizeWindowsPathDrive(project.folder, 'upper') }}
      </template>
      <template v-else>
        Launching...
      </template>
    </template>
    <template #favorite>
      <ui-button-icon
        size="small"
        title="Добавить в избранное"
        @click="emit('favorite')"
      >
        <iui-star :is-filled="project.is_favorite" />
      </ui-button-icon>
    </template>
    <template #settings>
      <w-project-dropdown
        @hide="emit('hidden')"
        @open-folder="emit('openFolder')"
      />
    </template>
  </ui-project>
</template>

<script setup lang="ts">
import type { ProjectProps } from '~~/layers/ui/app/components/project';

interface Project {
  name: string;
  folder: string;
  is_favorite: boolean;
  last_modified_timestamp: number;
}

interface Emits {
  open: [void];
  favorite: [void];
  hidden: [void];
  openFolder: [void];
}

interface Props extends ProjectProps {
  project: Project;
  isLaunching?: boolean;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

function getTwoLettersFromDirectoryName(directoryName: string): string {
  const words = directoryName
    .split(/[-._@\s]/)
    .filter((word) => word.trim() !== '');

  if (words.length === 0) {
    return '';
  }

  const firstWord = words.at(0)!;
  let result = firstWord.charAt(0).toUpperCase();

  if (words.length > 1) {
    const lastWord = words.at(-1)!;
    result += lastWord.charAt(0).toUpperCase();
  }

  return result;
}
</script>

<style scoped lang="scss">
.w-project {
  &__button {
    padding: 0;
    background: transparent;
    border: none;
    outline: none;
  }
}
</style>
