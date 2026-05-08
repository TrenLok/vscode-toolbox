<template>
  <nuxt-link
    :to="{ name: 'index' }"
    class="logo"
  >
    <slot />
  </nuxt-link>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.logo {
  outline: none;

  :deep(picture) {
    display: block;
    width: 3.77em;
    max-width: 3.77em;
    height: 1em;
  }
}
</style>
