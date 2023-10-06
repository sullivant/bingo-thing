<script setup lang="ts">
    import { defineComponent } from 'vue'
    import { onUpdated, ref } from 'vue'

    const props = defineProps(['ballData']);
    // var ballStates = new Array<boolean>();
    // onUpdated(() => {
    //     ballStates.value = props.ballData;
    // });

    function getBoard() {
        const board = [];
        for (let i = 0; i < 15; i++) { 
            const row = [];
            for (let j = 0; j < 5; j++) {
                const thisNumber = (i * 5 + j);
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
    board data in template <br/>
    <div class="bingo-board">
        <table>
            <thead>
                <tr>
                    <th>B</th>
                    <th>I</th>
                    <th>N</th>
                    <th>G</th>
                    <th>O</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(row, rowIndex) in getBoard()" :key="rowIndex">
                    <td v-for="(cell, colIndex) in row" :key="colIndex">
                        <div class="cell" :class="{ 'marked' : cell.marked }"> {{ cell.number }}</div>
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
.cell {
    /* cell styles */
}
.marked {
    background-color: #FF0000; /* for when a cell has a ball present */
}
</style>