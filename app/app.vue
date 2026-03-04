<template>
  <div class="page__main">
    <nuxt-layout>
      <nuxt-page />
    </nuxt-layout>
    <w-task-container />
  </div>

  <modals-container />
</template>

<script lang="ts" setup>
import { ModalsContainer } from 'vue-final-modal';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appStore = useAppStore();
const appSettings = useAppSettings();

const bus = useEventBus('blur-window');

const vscodeRecent = useVscodeRecent();
const {
  checkEqualsAndSyncVSCodeRecent,
} = useProjectManager();

let unwatch: null | (() => void) = null;

const windowFocusUnlisten = await getCurrentWindow().onFocusChanged(({ payload }) => {
  if (payload) return;
  bus.emit();
  appStore.scrollToTop();
});

onMounted(async () => {
  const rs = await vscodeRecent.watchVSCodeState(async () => {
    if (!appSettings.vsCodeSync.value) return;

    await checkEqualsAndSyncVSCodeRecent();
  }, 300);

  if (rs) unwatch = rs.unwatch;
});

onBeforeUnmount(() => {
  if (unwatch) unwatch();
  windowFocusUnlisten();
});
</script>

<style lang="scss">
@use 'perfect-scrollbar/css/perfect-scrollbar.css';

.ps {
  position: relative;
  max-width: 100vw;
  max-height: 100vh;
}

.ps__rail-y {
  z-index: 20;
  width: 3px;
  margin: 10px 3px;
  background-color: var(--gray_base_1) !important;
  border-radius: var(--border_1);
}

.ps__thumb-y {
  right: 0;
  width: 100% !important;
}

.ps__thumb-y,.ps__thumb-x {
  background-color: var(--gray_base_3) !important;
}
</style>

<style lang="scss" scoped>
.page {
  &__main {
    display: flex;
    flex-direction: column;
    gap: 10px;
    height: 100vh;
    padding: 10px;
  }
}
</style>
