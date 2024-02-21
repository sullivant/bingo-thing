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

<v-layout class="rounded rounded-md">
  <v-app-bar color="surface-variant" title="Bingo Thing"></v-app-bar>

  <v-navigation-drawer expand-on-hover rail>
      <v-divider></v-divider>

        <v-list density="compact" nav>
          <v-list-item
            prepend-icon="mdi-robot-love-outline"
            title="Controls"
          />
          <v-divider/>

          <v-list-item prepend-icon="mdi-memory" title="Update Ball States" @click="updateBallStates()"></v-list-item>
        </v-list>
    </v-navigation-drawer>


    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
        <v-container>
          <v-row>
            <v-col>
              <v-card class="mx-auto" prepend-icon="mdi-home">
                <template v-slot:title>BINGO</template>
              </v-card>
            </v-col>
          </v-row>
          <!-- <v-row> -->
            <board :ballData="ballStates" />
          <!-- </v-row> -->
      </v-container>
    </v-main>

    <v-bottom-navigation>
      <v-footer>
        {{ new Date().getFullYear() }} - Winsted Fire Department
      </v-footer>
    </v-bottom-navigation> 

</v-layout>

</template>

<style scoped>

</style>
