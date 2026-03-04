<template>
  <div :class="classNames({ ...props })">
    <div class="project__wrapper">
      <button type="button" class="project__button" @click="emit('click')">
        <span class="project__icon">
          <slot name="icon" />
        </span>
        <span class="project__body">
          <span class="project__title">
            <slot name="title" />
          </span>
          <span class="project__subtitle">
            <slot name="subtitle" />
          </span>
        </span>
      </button>
      <div class="project__buttons">
        <div class="project__favorite">
          <slot name="favorite" />
        </div>
        <div class="project__settings">
          <slot name="settings" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { ProjectEmits, ProjectProps } from '.';

const emit = defineEmits<ProjectEmits>();
const props = defineProps<ProjectProps>();

const classNames = bmc<ProjectProps>('project', {
  modifiers: {
    inactive: { modifier: 'state', stateIfTrue: 'inactive' },
  },
  whitelist: ['inactive'],
});
</script>

<style lang="scss" scoped>
.project {
  --project_buttons__opacity: 0;
  --project_wrapper__opacity: 1;
  --project_icon__size: 35px;

  width: 100%;
  max-width: 100%;
  padding: 5px;
  cursor: pointer;
  user-select: none;
  border-radius: 8px;
  transition: background-color .3s ease;

  &:hover,
  &:focus-visible {
    --project_buttons__opacity: 1;

    background-color: var(--gray_base_0);
  }

  &__wrapper {
    display: flex;
    gap: 10px;
    align-items: flex-start;
    opacity: var(--project_wrapper__opacity);
  }

  &__button {
    display: flex;
    flex-grow: 1;
    gap: 10px;
    align-items: flex-start;
  }

  &__icon {
    display: grid;
    flex-shrink: 0;
    width: var(--project_icon__size);
    height: var(--project_icon__size);
    color: var(--text_accent);
    font-size: 14px;
    font-weight: 600;
    line-height: normal;
    background-color: var(--gray_base_2);
    border-radius: var(--border_2);
    place-content: center;
  }

  &__body {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    text-align: left;
  }

  &__title {
    overflow: hidden;
    font-weight: 400;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  &__subtitle {
    overflow: hidden;
    color: var(--gray_base_8);
    font-size: 12px;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  &__buttons {
    display: flex;
    flex-shrink: 0;
    gap: 2px;
    align-items: center;
    transition: opacity .3s ease;
  }

  &__favorite {
    opacity: var(--project_buttons__opacity);
  }

  &_state {
    &_inactive {
      --project_wrapper__opacity: .4;
    }
  }
}
</style>
