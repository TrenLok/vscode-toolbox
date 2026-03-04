<template>
  <div class="modal-container">
    <p class="modal-container__title">
      <slot name="title" />
    </p>
    <div class="modal-container__body">
      <slot />
    </div>
    <template v-if="$slots.actions">
      <div class="modal-container__actions">
        <slot name="actions" />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 5px;
  width: var(--modal-container__width, 350px);
  padding: 20px;
  background-color: #1f1f1f;
  border: 2px solid rgb(255, 109, 109);
  border-radius: 8px;

  &__title {
    font-weight: 600;
  }

  &__actions {
    display: flex;
    gap: 10px;
    align-items: center;
    padding-top: 10px;
  }
}
</style>
