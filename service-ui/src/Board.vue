<script setup lang="ts">
    const props = defineProps(['ballData']);

    const letters = ["B","I","N","G","O"];

    function getBoard() {
        const board = [];

        // For 5 rows (B I N G O)
        for (let i = 0; i < 5; i++) { 
            const row = [];
            // Push the letter
            row.push({number: letters[i], marked: false, class: "ballHeader"});

            // For 15 columns
            for (let j = 0; j < 15; j++) {
                const thisNumber = (i*15) + j;
                const cellData = props.ballData[thisNumber];
                row.push({
                    number: thisNumber+1,
                    marked: cellData > 0 ? true : false,
                    class: "ballCard",
                })
            }
            board.push(row);
        }
        return board;
    }
    
</script>

<template>
    <!-- <v-card class="mx-auto" max-width="100%"> -->
        <!-- <v-container fluid> -->
            <v-row v-for="(row, rowIndex) in getBoard()" :key="rowIndex" dense>
                <v-col v-for="(cell, colIndex) in row" :key="colIndex" :cols="16">
                    <v-card>
                        <v-card-title v-text=cell.number :class="[cell.class, { 'ballMarked' : cell.marked }]"></v-card-title>
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
.ballMarked {
    background-color: #FF0000; /* for when a cell has a ball present */
}
.ballCard {
    text-align: center;
}
</style>