<template>
  <div class="task">
    <slot />
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.task {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px 10px;
  background-color: var(--gray_base_1);
  border: 1px solid var(--primary);
  border-radius: var(--border_2);
}
</style>
