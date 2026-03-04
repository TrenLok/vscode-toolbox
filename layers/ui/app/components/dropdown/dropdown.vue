<template>
  <div
    role="presentation"
    ref="root"
    :class="classNames({ ...props })"
    @keydown="onKeyDown"
    @mouseenter="isHovered = true"
    @focus="isHovered = true"
    @mouseleave="isHovered = false"
    @blur="isHovered = false"
  >
    <div ref="buttonRoot" class="dropdown__button">
      <!-- eslint-disable-next-line vue/attribute-hyphenation -->
      <slot name="button" :toggle="toggle" :isOpened="isOpened || (isHovered && openOnHover)" />
    </div>
    <transition name="fade">
      <div v-show="isOpened || (isHovered && openOnHover)" ref="contentRoot" class="dropdown__content">
        <slot name="content" :close="close" />
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import type { DropdownProps } from '.';

const root = ref<HTMLElement>();
const contentRoot = ref<HTMLElement>();
const buttonRoot = ref<HTMLElement>();

const isOpened = ref(false);
const isHovered = ref(false);

const props = withDefaults(defineProps<DropdownProps>(), {
  contentPositionX: undefined,
  contentPositionY: 'bottom',
  openOnHover: false,
  width: 'auto',
});

const classNames = bmc<DropdownProps>('dropdown', {
  modifiers: {
    contentPositionY: 'position-y',
    contentPositionX: 'position-x',
  },
  whitelist: ['contentPositionX', 'contentPositionY', 'width'],
});

function toggle(): void {
  isOpened.value = !isOpened.value;
}

function close(): void {
  isOpened.value = false;
  isHovered.value = false;
  focusButton();
}

function onDocumentClick(event: MouseEvent): void {
  const target = event.target as Node;
  if (!root.value?.contains(target) && isOpened.value) {
    isOpened.value = false;
  }
}

function focusButton(): void {
  const button = buttonRoot.value?.querySelector('button') as HTMLButtonElement | undefined;
  if (!button) return;

  button.focus();
}

interface GetContentButtonsReturnType {
  elements: HTMLButtonElement[];
  first: HTMLButtonElement | undefined;
  last: HTMLButtonElement | undefined;
}

function getContentButtons(): GetContentButtonsReturnType {
  let elements: HTMLButtonElement[] = [];

  if (contentRoot.value) {
    elements = [...contentRoot.value.querySelectorAll('button')] as HTMLButtonElement[];
  }

  return {
    elements,
    first: elements[0],
    last: elements.at(-1),
  };
}

function onKeyDown(event: KeyboardEvent): void {
  if (event.code === 'Escape' && isOpened.value) {
    close();
    event.preventDefault();
  }

  if (event.code === 'Tab' && contentRoot.value) {
    const { first, last } = getContentButtons();

    if (event.shiftKey) {
      if (first && event.target === first) {
        last?.focus();
        event.preventDefault();
      }
    } else if (last && event.target === last) {
      first?.focus();
      event.preventDefault();
    }
  }
}

defineExpose({
  close,
});

onMounted(() => {
  document.addEventListener('click', onDocumentClick);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', onDocumentClick);
});
</script>

<style lang="scss">
.fade {
  &-enter-active,
  &-leave-active {
    opacity: 1;
    transition: opacity .1s;
  }

  &-enter-from,
  &-leave-to {
    opacity: 0;
  }
}

.dropdown {
  --dropdown-content__content-offset: 10px;

  position: relative;
  min-width: var(--dropdown__min-width, 0);

  &__button {
    display: flex;
    width: 100%;
    height: 100%;
  }

  &__content {
    position: absolute;
    right: 0;
    left: 0;
    z-index: 100;
    overflow: hidden;
    width: var(--dropdown__content-width, unset);
    margin-left: var(--dropdown-content__offset-x, 0);
  }

  &_position-x {
    &_left {
      .dropdown__content {
        right: unset;
        left: 0;
      }
    }

    &_right {
      .dropdown__content {
        right: 0;
        left: unset;
      }
    }

    &_center {
      .dropdown__content {
        right: unset;
        left: 50%;
        transform: translateX(-50%);
      }
    }
  }

  &_position-y {
    &_top {
      .dropdown__content {
        top: unset;
        bottom: 100%;
        padding-bottom: var(--dropdown-content__content-offset);
      }
    }

    &_bottom {
      .dropdown__content {
        top: 100%;
        bottom: unset;
        padding-top: var(--dropdown-content__content-offset);
      }
    }
  }

  &_width {
    &_default {
      width: 350px;
    }

    &_auto {
      width: auto;
    }
  }
}
</style>
