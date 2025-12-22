const START_STONE_VALS = [4, 4841539, 66, 5279, 49207, 134, 609568, 0]
const START_STONES = []
const N_BLINKS = 75

const STONES_TO_VISIT = []
const VISITED_STONES = []
let TOTAL = 0

START_STONE_VALS.forEach(stoneVal => {
    const stone = {
        value: stoneVal,
        depth: 0,
        nextStones: []
    }
    START_STONES.push(stone)
    STONES_TO_VISIT.push(stone)
})

const nDigits = val => {
    return Math.floor((Math.log(val)) / Math.log(10)) + 1
}

const visitedBefore = (stone) => {
    const matches = VISITED_STONES
        .filter(s => s.value == stone.value)
        .filter(s => s.depth <= stone.depth)

    return matches.length > 0
}

const STONE_GRAPH = {}

const findVisitedStone = (value, depth) => {
    return VISITED_STONES.filter(stone => stone.value == value).filter(stone => stone.depth == depth)[0]
}

while (STONES_TO_VISIT.length > 0) {
    TOTAL += 1
    // console.log(STONES_TO_VISIT)
    const stone = STONES_TO_VISIT.reduce((acc, curr) => curr.depth < acc.depth ? curr : acc, { depth: N_BLINKS + 1 })
    // console.log(stone)
    const stoneIndex = STONES_TO_VISIT.findIndex(s => s.value == stone.value && s.depth == stone.depth)
    // console.log(stoneIndex)

    STONES_TO_VISIT.splice(stoneIndex, 1)
    VISITED_STONES.push(stone)
    const nextStones = []
    // console.log(STONES_TO_VISIT)

    if (stone.value == 0) {
        const nextStone = {
            value: 1,
            depth: stone.depth + 1
        }
        nextStones.push(nextStone.value)
        if (!visitedBefore(nextStone) && nextStone.depth != N_BLINKS) {
            STONES_TO_VISIT.push(nextStone)
        }
        VISITED_STONES.push(nextStone)
    } else if (`${stone.value}`.length % 2 == 0) {
        const len = `${stone.value}`.length
        const nextStone1 = {
            value: Number(`${stone.value}`.substring(0, len / 2)),
            depth: stone.depth + 1
        }
        nextStones.push(nextStone1.value)
        if (!visitedBefore(nextStone1) && nextStone1.depth != N_BLINKS) {
            STONES_TO_VISIT.push(nextStone1)
        }
        const nextStone2 = {
            value: Number(`${stone.value}`.substring(len / 2)),
            depth: stone.depth + 1
        }
        nextStones.push(nextStone2.value)
        if (!visitedBefore(nextStone2) && nextStone2.depth != N_BLINKS) {
            STONES_TO_VISIT.push(nextStone2)
        }
        VISITED_STONES.push(nextStone1)
        VISITED_STONES.push(nextStone2)
    } else {
        const nextStone = {
            value: stone.value * 2024,
            depth: stone.depth + 1
        }
        nextStones.push(nextStone.value)
        if (!visitedBefore(nextStone) && nextStone.depth != N_BLINKS) {
            STONES_TO_VISIT.push(nextStone)
        }
        VISITED_STONES.push(nextStone)
    }

    if (!STONE_GRAPH[stone.value]) {
        STONE_GRAPH[stone.value] = nextStones
    }
}

const calculateStoneTotal = (stone) => {
    if (stone.total) {
        return stone.total
    }
    if (stone.depth == N_BLINKS) {
        stone.total = 1
        return 1
    }
    const nextStones = STONE_GRAPH[stone.value]
    const total = nextStones.reduce((acc, curr) => {
        const visitedStone = findVisitedStone(curr, stone.depth + 1)
        
        if (visitedStone) {
            // console.log(visitedStone)
            return acc + calculateStoneTotal(visitedStone)
        }

        const newStone = {
            value: curr,
            depth: stone.depth + 1
        }
        VISITED_STONES.push(newStone)

        return acc + calculateStoneTotal(newStone)
    }, 0)
    stone.total = total
    return total
}

while (VISITED_STONES.filter(s => !s.total).length > 0) {
    console.log(VISITED_STONES.filter(s => !s.total).length)
    const stone = VISITED_STONES.filter(s => !s.total).reduce((acc, curr) => curr.depth > acc.depth ? curr : acc, { depth: -1 })
    calculateStoneTotal(stone)
}

START_STONES.forEach(console.log)

const total = START_STONES.reduce((acc, curr) => {
    return acc + curr.total
}, 0)
console.log(total)