<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { useFetch } from '@vueuse/core'
import { useToggle } from '@vueuse/shared'

let url = 'http://service-ball:8000/balls';

function getPosts() {
  fetch(url)
  .then(response => response.json())
  .then(data => console.log(data));
}

</script>

<template>
  <header>
    This is a simple header. {{ url }}
  </header>

  <main>
    This is main content.

    <button @click="getPosts">Show Balls</button>

  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

}
</style>
