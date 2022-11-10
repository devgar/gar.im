<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<template>
  <div class="greetings">
    <!--<h1 class="green">{{ msg }}</h1>-->
    <h3>
      This is a simple tool made by
      <a href="https://ed.gar.im/" target="_blank" rel="noopener">Edgar Albalate</a>
       mainly aimed at helping to share content for job research purposes.
      <!--<a href="https://github.com/devgar/gar.im/" target="_blank" rel="noopener">github</a>. -->
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
