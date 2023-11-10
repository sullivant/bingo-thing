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
    <div class="bingo-board">
        <table>

            <tbody>
                <tr v-for="(row, rowIndex) in getBoard()" :key="rowIndex">
                    <td v-for="(cell, colIndex) in row" :key="colIndex">
                        <div :class="[cell.class, { 'ballMarked' : cell.marked }]"> {{ cell.number }}</div>
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
    font-weight: bold;
}
.ballMarked {
    background-color: #FF0000; /* for when a cell has a ball present */
}
</style>