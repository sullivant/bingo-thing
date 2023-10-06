<script setup lang="ts">
import { onBeforeMount, onMounted, ref } from 'vue'
import { useFetch } from '@vueuse/core'
import { useToggle } from '@vueuse/shared'
import board from './Board.vue'

let url = 'http://service-ball:8000/balls';

const ballStates: any = ref([])

async function fetchBallStates() {
  console.log("Fetching ball states");
  const res = await fetch(url);
  const data = await res.json();
  const newObject = data;
  console.log("New ball states: ", newObject);
  return newObject;
}

async function updateBallStates() {
  const newStates = await fetchBallStates();
  ballStates.value = newStates;
}

onMounted(async () => {
  // const newStates = await fetchBallStates();
  // ballStates.value = newStates;
  await updateBallStates();
})

</script>

<template>
  <header>
    This is a simple header. {{ url }}
  </header>

  <main>
    This is main content.

    <button @click="updateBallStates()">Update Ball States</button>

    <div>
      <board :ballData="ballStates" />
    </div>
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
