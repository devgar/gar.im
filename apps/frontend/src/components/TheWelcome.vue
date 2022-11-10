<script setup lang="ts">
import WelcomeItem from './WelcomeItem.vue'
</script>

<template>
  <h2>Recent Links</h2>
  <WelcomeItem v-for="i in 5" key="i">
    <template #heading>__LINK_SLUG__ /some-link-to</template>
    <div><pre>__URL_TO_LINK__ (https://)</pre></div>
    <h3>__DOCUMENT_TITLE__</h3>
  </WelcomeItem>
</template>

<style scoped>
h2 {
  padding-left:  calc(var(--section-gap) / 3);
  margin-left: 1rem;
}
</style>