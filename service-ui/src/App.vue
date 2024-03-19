<script setup lang="ts">
import { onMounted, ref } from 'vue'
import board from './Board.vue'

let url = 'http://service-ball:8000/balls';

const ballStates: any = ref([])
let doSimulate: boolean = true;

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

async function simulateBallStates() {
  if (!doSimulate) { return; }
  await updateBallStates();
}

onMounted(async () => {
  // await simulateBallStates();

  let pollInterval = setInterval(simulateBallStates, 2000) 
  // setTimeout(() => { clearInterval(pollInterval) }, 5000) 
})

function clearAllBalls() {
  ballStates.value.map(function(value :any, key :any) {
    ballStates.value[key] = 0;
  })
}

function toggleSimulate() {
  doSimulate = !doSimulate;
}

</script>

<template>

<v-layout class="rounded rounded-md">
  <!-- <v-app-bar color="surface-variant" title="Bingo Thing"></v-app-bar> -->

  <v-navigation-drawer expand-on-hover rail>
      <v-divider></v-divider>

        <v-list density="compact" nav>
          <v-list-item
            prepend-icon="mdi-robot-love-outline"
            title="Controls"
          />
          <v-divider/>
          <v-list-item prepend-icon="mdi-refresh" title="Update Ball States" @click="updateBallStates()"></v-list-item>
          <v-list-item prepend-icon="mdi-lightbulb-off-outline" title="Clear Ball States" @click="clearAllBalls()"></v-list-item>
          <v-divider/>
          
        </v-list>
    </v-navigation-drawer>


    <v-main class="d-flex align-center justify-center" style="min-height: 300px;">
        <v-container>
          <v-row>
            <v-col>
              <v-card class="mx-auto" color="grey-lighten-2">
                <v-card-title>
                  <v-icon icon="mdi-fire-truck" color="red" size="small"></v-icon> Winsted Fire Department - BINGO</v-card-title>
              </v-card>
            </v-col>
          </v-row>
          <!-- <v-row> -->
          <board :ballData="ballStates" />
          <!-- </v-row> -->

          <v-row>
            <v-carousel :show-arrows="false" delimiter-icon="mdi-square" cycle hide-delimiter-background >
              <v-carousel-item
                src="https://cdn.vuetifyjs.com/images/cards/docks.jpg"
                cover
              ></v-carousel-item>

              <v-carousel-item
                src="https://cdn.vuetifyjs.com/images/cards/hotel.jpg"
                cover
              ></v-carousel-item>

              <v-carousel-item
                src="https://cdn.vuetifyjs.com/images/cards/sunshine.jpg"
                cover
              ></v-carousel-item>
            </v-carousel>
          </v-row>
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
