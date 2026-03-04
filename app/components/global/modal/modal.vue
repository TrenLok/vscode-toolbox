<template>
  <vue-final-modal
    v-bind="$attrs"
    overlay-transition="vfm-fade"
    content-transition="vfm-fade"
    class="modal"
    :click-to-close="true"
    :esc-to-close="true"
    :lock-scroll="false"
  >
    <slot />
  </vue-final-modal>
</template>

<script setup lang="ts">
import { VueFinalModal } from 'vue-final-modal';
</script>

<style scoped lang="scss">

</style>
