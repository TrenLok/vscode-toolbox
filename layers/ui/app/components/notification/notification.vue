<template>
  <button
    type="button"
    :class="classNames({ ...props })"
    @click="emit('close')"
  >
    <div class="notification__body">
      <p class="notification__title">
        <slot name="title" />
      </p>
      <p class="notification__text">
        <slot name="text" />
      </p>
    </div>
  </button>
</template>

<script setup lang="ts">
import type { NotificationEmits, NotificationProps } from '.';

const props = withDefaults(defineProps<NotificationProps>(), {
  variant: 'default',
});

const emit = defineEmits<NotificationEmits>();

const classNames = bmc<NotificationProps>('notification', {
  variant: true,
});
</script>

<style scoped lang="scss">
.notification {
  display: flex;
  gap: 10px;
  align-items: center;
  width: 100%;
  padding: 10px;
  text-align: left;
  background-color: var(--notification__background-color);
  border-radius: var(--border_1);
  transition: background-color .3s ease;

  &__body {
    color: var(--text_accent);
  }

  &__title {
    font-weight: 500;
  }

  &_variant {
    &_default {
      --notification__background-color: var(--primary);
    }

    &_success {
      --notification__background-color: var(--green);
    }

    &_error {
      --notification__background-color: var(--red);
    }
  }
}
</style>
