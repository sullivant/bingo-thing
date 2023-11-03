<script setup lang="ts">
import { onMounted, ref } from 'vue'
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
  await updateBallStates();

  let pollInterval = setInterval(updateBallStates, 1000) 
  // setTimeout(() => { clearInterval(pollInterval) }, 5000) 
})

</script>

<template>
  <main>
    <div>
      <board :ballData="ballStates" />
    </div>
    <button @click="updateBallStates()">Update Balls</button>

  </main>
</template>

<style scoped>

</style>
