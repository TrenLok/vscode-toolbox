<template>
  <div class="segment-control">
    <template v-if="$slots.label">
      <span class="segment-control__label">
        <slot name="label" />
      </span>
    </template>
    <div class="segment-control__body">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.segment-control {
  display: flex;
  gap: 10px;
  align-items: center;

  &__body {
    display: inline-block;
    width: max-content;
  }
}
</style>
