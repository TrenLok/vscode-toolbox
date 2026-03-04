<template>
  <b-layout>
    <template #header>
      <w-header>
        <nuxt-link
          :to="{ name: 'index' }"
          v-slot="{ href, navigate }"
          custom
        >
          <ui-button-primary size="default" :href="href" @click="navigate($event);">
            <template #icon>
              <iui-angle-left />
            </template>
            <template #default>
              Projects
            </template>
          </ui-button-primary>
        </nuxt-link>
      </w-header>
    </template>
    <template #default>
      <slot />
    </template>
  </b-layout>
</template>

<style scoped lang="scss">
</style>
