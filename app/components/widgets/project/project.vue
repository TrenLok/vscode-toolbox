<template>
  <ui-project
    class="w-project"
    :inactive="inactive"
    @click="emit('open')"
  >
    <template #icon>
      <ui-project-icon :background="iconBackground">
        {{ getTwoLettersFromDirectoryName(project.name) }}
      </ui-project-icon>
    </template>
    <template #title>
      {{ projectTitle.name }}
      <b v-if="projectTitle.badge">{{ projectTitle.badge }}</b>
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
        :is-remote="isRemote"
        @hide="emit('hidden')"
        @open-folder="emit('openFolder')"
      />
    </template>
  </ui-project>
</template>

<script setup lang="ts">
import type { ProjectProps } from '~~/layers/ui/app/components/project';
import type { ProjectType } from '~/types/project';

interface Project {
  type?: ProjectType;
  name: string;
  folder: string;
  uri?: string;
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

interface ProjectTitle {
  name: string;
  badge: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const { getGradientFromString } = useStringColor();
const appSettings = useAppSettings();

const iconBackground = computed(() => {
  if (appSettings.projectIconStyle.value !== 'default') {
    return getGradientFromString(props.project.name);
  }

  return;
});
const isRemote = computed(() => isVSCodeRemoteUri(props.project.uri ?? props.project.folder));
const projectTitle = computed(() => getProjectTitle(props.project.name, props.project.type));

function getProjectTitle(projectName: string, type?: ProjectType): ProjectTitle {
  const coderPattern = /\s*(\[Coder:[^\]]+\])/;
  const coder = coderPattern.exec(projectName)
    ?.at(1)
    ?.replaceAll('∕', '/') ?? '';
  const name = projectName.replace(coderPattern, '').trim();
  const badge = getProjectBadge(coder, type);

  return { name, badge };
}

function getProjectBadge(coder: string, type?: ProjectType): string {
  if (coder && type === 'workspace') return coder.replace('[Coder:', '[Workspace Coder:');
  if (coder) return coder;
  if (type === 'workspace') return '[Workspace]';

  return '';
}

function getTwoLettersFromDirectoryName(directoryName: string): string {
  const words = directoryName
    .replaceAll(/\s*\[[^\]]+\]/g, '')
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
