<template>
  <b-layout>
    <template #header>
      <w-header>
        <template v-if="appUpdate.latestUpdate.value">
          <nuxt-link
            :to="{ name: 'settings' }"
            v-slot="{ href, navigate }"
            custom
          >
            <ui-button-icon
              size="default"
              title="Update available"
              :href="href"
              @click="navigate($event);"
            >
              <iui-cloud-arrow-down />
            </ui-button-icon>
          </nuxt-link>
        </template>

        <nuxt-link
          :to="{ name: 'settings' }"
          v-slot="{ href, navigate }"
          custom
        >
          <ui-button-icon size="default" :href="href" @click="navigate($event);">
            <iui-settings />
          </ui-button-icon>
        </nuxt-link>
      </w-header>
    </template>
    <template #default>
      <slot />
    </template>
  </b-layout>
</template>

<script setup lang="ts">
const appUpdate = useAppUpdate();
</script>

<style scoped lang="scss">
</style>
