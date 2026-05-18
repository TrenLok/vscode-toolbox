<template>
  <div class="page__main">
    <nuxt-layout>
      <nuxt-page />
    </nuxt-layout>
    <w-task-container />
  </div>

  <nuxt-notifications
    class="notifications"
    classes="notifications__notification"
    position="bottom center"
    reverse
    :duration="5000"
    :speed="200"
    :max="10"
    width="95%"
    :pause-on-hover="true"
  >
    <template #body="props">
      <ui-notification
        :variant="props.item.type"
        @close="props.close"
      >
        <template #title v-if="props.item.title">
          {{ props.item.title }}
        </template>
        <template #text>
          {{ props.item.text }}
        </template>
      </ui-notification>
    </template>
  </nuxt-notifications>
  <modals-container />
</template>

<script lang="ts" setup>
import { ModalsContainer } from 'vue-final-modal';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appStore = useAppStore();
const appSettings = useAppSettings();

const blurBus = useEventBus('blur-window');
const focusBus = useEventBus('focus-window');

const vscodeRecent = useVscodeRecent();
const {
  checkEqualsAndSyncVSCodeRecent,
} = useProjectManager();

let unwatch: null | (() => void) = null;
let blurTimeout: null | ReturnType<typeof setTimeout> = null;

function clearBlurTimeout() {
  if (!blurTimeout) return;
  clearTimeout(blurTimeout);
  blurTimeout = null;
}

const windowFocusUnlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
  if (focused) {
    clearBlurTimeout();
    focusBus.emit();
    return;
  }

  clearBlurTimeout();
  blurTimeout = setTimeout(() => {
    blurBus.emit();
    appStore.scrollToTop();
    blurTimeout = null;
  }, 120);
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
  clearBlurTimeout();
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

.notifications {
  position: fixed;
  margin: 10px 0;

  & > :deep(div) {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
}
</style>
