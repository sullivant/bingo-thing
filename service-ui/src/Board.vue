<script setup lang="ts">
    const props = defineProps(['ballData']);

    function getBoard() {
        const board = [];

        // For 15 rows
        for (let i = 0; i < 15; i++) { 
            const row = [];
            // For 5 columns (B,I,N,G,O)
            for (let j = 0; j < 5; j++) {
                const thisNumber = (i + 15 * j);
                const cellData = props.ballData[thisNumber];
                row.push({
                    number: thisNumber+1,
                    marked: cellData > 0 ? true : false,
                })
            }
            board.push(row);
        }
        return board;
    }
    
</script>

<template>
    <div class="bingo-board">
        <table>
            <thead>
                <tr>
                    <th class="ballHeader">B</th>
                    <th>I</th>
                    <th>N</th>
                    <th>G</th>
                    <th>O</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(row, rowIndex) in getBoard()" :key="rowIndex">
                    <td v-for="(cell, colIndex) in row" :key="colIndex">
                        <div class="ballCard" :class="{ 'marked' : cell.marked }"> {{ cell.number }}</div>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

</template>


<style scoped>
.bingo-board {
    /* bingo board styles */
}
.ballCard {
    /* cell styles */
}
.ballHeader {
    
}
.marked {
    background-color: #FF0000; /* for when a cell has a ball present */
}
</style>