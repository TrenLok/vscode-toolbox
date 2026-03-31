<template>
  <div class="main-page">
    <template v-if="hasProjects || search.length">
      <ui-button-primary width="full" @click="openNewProject">
        Open a new project
      </ui-button-primary>
      <ui-input
        v-model="search"
        is-clearable
        placeholder="Search"
      >
        <template #icon-after>
          <iui-search />
        </template>
      </ui-input>
      <div ref="scrollContainer" class="main-page__projects">
        <template v-if="!hasProjects && search.length">
          <p class="main-page__not-found">
            No projects found
          </p>
        </template>
        <template v-else>
          <template v-if="filteredProjects.favorites.length">
            <b-project-list>
              <template #title>
                Favorite
              </template>
              <template #default>
                <template v-for="project of filteredProjects.favorites" :key="project.folder">
                  <w-project
                    :project="project"
                    :inactive="badFolders.has(project.folder)"
                    @open="openProjectFolder(project.folder)"
                    @favorite="changeFavorite(project.folder)"
                    @hidden="modals.folderHidden(project)"
                    @open-folder="openProjectInExplorer(project.folder)"
                  />
                </template>
              </template>
            </b-project-list>
          </template>
          <template v-if="filteredProjects.local.length">
            <b-project-list>
              <template #title>
                local
              </template>
              <template #default>
                <template v-for="project of filteredProjects.local" :key="project.folder">
                  <w-project
                    :project="project"
                    :inactive="badFolders.has(project.folder)"
                    @open="openProjectFolder(project.folder)"
                    @favorite="changeFavorite(project.folder)"
                    @hidden="modals.folderHidden(project)"
                    @open-folder="openProjectInExplorer(project.folder)"
                  />
                </template>
              </template>
            </b-project-list>
          </template>
        </template>
      </div>
    </template>
    <template v-else>
      <div class="main-page__empty">
        <b-empty-projects @open-project="openNewProject" />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Project } from '~/types/project';
import PerfectScrollbar from 'perfect-scrollbar';

const {
  projects,
  badFolders,
  changeFavorite,
  openProjectFolder,
  openNewProject,
} = useProjectManager();

const appStore = useAppStore();

const hiddenFolders = useHiddenFolders();
const modals = useModals();

const search = ref('');

const bus = useEventBus('blur-window');

const stop = bus.on(() => {
  search.value = '';
});

const filteredProjects = computed(() => {
  const { favorites, local } = projects.value;
  const query = search.value.toLowerCase();

  const isVisible = (project: Project) =>
    !hiddenFolders.hasFolder(project.folder);

  const matchesSearch = (project: Project) =>
    project.name.toLowerCase().includes(query)
    || project.folder.toLowerCase().includes(query);

  const filterProject = (project: Project) =>
    isVisible(project) && matchesSearch(project);

  return {
    favorites: favorites.filter((project) => filterProject(project)),
    local: local.filter((project) => filterProject(project)),
  };
});

const hasProjects = computed(() => filteredProjects.value.favorites.length || filteredProjects.value.local.length);

async function openProjectInExplorer(folder: string) {
  try {
    await useTauriOpenerOpenPath(folder);
  } catch (error_) {
    useTauriLogError(`Couldn't open folder: ${error_}`);
  }
}

const perfectScroll = shallowRef<PerfectScrollbar>();
const scrollContainer = useTemplateRef('scrollContainer');

function initScrollbar() {
  if (!scrollContainer.value) return;
  appStore.projectsScrollContainer = scrollContainer.value;

  perfectScroll.value = new PerfectScrollbar(scrollContainer.value);
}

function cleanupScrollbar() {
  perfectScroll.value?.destroy();
  perfectScroll.value = undefined;
  appStore.projectsScrollContainer = null;
}

watch([filteredProjects], async () => {
  await nextTick();
  perfectScroll.value?.update();
});

onMounted(() => {
  initScrollbar();
});

onBeforeUnmount(() => {
  cleanupScrollbar();
  stop();
});
</script>

<style scoped lang="scss">
.main-page {
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: flex-start;
  height: 100%;
  max-height: 100%;

  &__projects {
    overflow-y: scroll;
    width: 105%;
    height: 100%;
    margin: 0 -10px;
    padding: 0 10px;
  }

  &__not-found {
    width: 100%;
    padding: 5px 0;
    color: var(--gray_base_8);
    text-align: center;
  }

  &__empty {
    display: grid;
    width: 100%;
    height: 100%;
    place-content: center;
  }
}
</style>
