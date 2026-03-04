<template>
  <iui-svg
    v-slot="{ color }"
    width="14"
    height="8"
    viewBox="0 0 14 8"
  >
    <path d="M7 7.5C6.71875 7.5 6.46875 7.40625 6.28125 7.21875L1.28125 2.21875C0.875 1.84375 0.875 1.1875 1.28125 0.8125C1.65625 0.40625 2.3125 0.40625 2.6875 0.8125L7 5.09375L11.2812 0.8125C11.6562 0.40625 12.3125 0.40625 12.6875 0.8125C13.0938 1.1875 13.0938 1.84375 12.6875 2.21875L7.6875 7.21875C7.5 7.40625 7.25 7.5 7 7.5Z" :fill="color" />
  </iui-svg>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.icon {
  width: 1em;
  height: 1em;
}
</style>
