<template>
  <iui-svg
    width="512"
    height="512"
    viewBox="0 0 512 512"
    v-slot="{ color }"
  >
    <path :fill="color" d="M448 448L64 448c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l138.7 0c13.8 0 27.3 4.5 38.4 12.8l38.4 28.8c5.5 4.2 12.3 6.4 19.2 6.4L448 80c35.3 0 64 28.7 64 64l0 240c0 35.3-28.7 64-64 64zM184 240c-13.3 0-24 10.7-24 24s10.7 24 24 24l144 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-144 0z" />
  </iui-svg>
</template>

<style scoped lang="scss">
.icon {
  width: 1em;
  height: 1em;
}
</style>
