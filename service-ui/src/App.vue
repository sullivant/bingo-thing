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

<v-layout>
  <v-main>
    <v-card
      class="mx-auto"
      width="400"
      prepend-icon="mdi-home"
    >
      <template v-slot:title>
        This is a title
      </template>

      <v-card-text>
        This is content
      </v-card-text>
    </v-card>

    <div>
      <board :ballData="ballStates" />
    </div>
    <button @click="updateBallStates()">Update Balls</button>
</v-main>
</v-layout>
</template>

<style scoped>

</style>
