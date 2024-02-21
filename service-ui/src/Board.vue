<script setup lang="ts">
    const props = defineProps(['ballData']);

    const letters = ["B","I","N","G","O"];

    function getBoard() {
        const board = [];

        // For 5 rows (B I N G O)
        for (let i = 0; i < 5; i++) { 
            const row = [];
            // Push the letter
            row.push({letter: letters[i], number: -1, marked: false, class: "ballHeader", color: "grey-lighten-4"});

            // For 15 columns
            for (let j = 0; j < 15; j++) {
                const thisNumber = (i*15) + j;
                const cellData = props.ballData[thisNumber];
                row.push({
                    number: thisNumber+1,
                    marked: cellData > 0 ? true : false,
                    class: "ballCard",
                    color: cellData > 0 ? "red-lighten-2" : "lime-lighten-5"
                })
            }
            board.push(row);
        }
        return board;
    }

    function toggleCard(cell: number) {
        props.ballData[cell] = props.ballData[cell] > 0 ? 0 : 1;
    }
    
</script>

<template>
    <!-- <v-card class="mx-auto" max-width="100%"> -->
        <!-- <v-container fluid> -->
            <v-row v-for="(row, rowIndex) in getBoard()" :key="rowIndex" dense>
                <v-col v-for="(cell, colIndex) in row" :key="colIndex" :cols="16">
                    <v-card elevation="4" key="foo" :color=cell.color variant="flat" :class="cell.class" link @click="toggleCard(cell.number-1)">
                        <v-card-title v-text="cell.number < 0 ? cell.letter : cell.number"></v-card-title>
                    </v-card>
                </v-col>
            </v-row>
        <!-- </v-container> -->
    <!-- </v-card> -->
</template>


<style scoped>
.ballHeader {
    font-weight: bold;
    text-align: center;
}
.ballCard {
    text-align: center;
}
</style>