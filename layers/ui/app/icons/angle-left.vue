<template>
  <iui-svg
    width="256"
    height="512"
    viewBox="0 0 256 512"
    v-slot="{ color }"
  >
    <path :fill="color" d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256 214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z" />
  </iui-svg>
</template>

<style scoped lang="scss">
.icon {
  width: .5em;
  height: 1em;
}
</style>
