let start = new Date().getTime()
let players = 418 
let rounds = 71339 * 100

let scores = new Array(players).fill(null).map(s => BigInt(0))
let marbles = [0]
let current = 0

for(let round = 1; round <= rounds; round++) {
    if(round % 23) {
        let dist = (round == 1) ? 1 : 2
        let index = (current + dist)
        if(index > marbles.length) index -= marbles.length
        marbles.splice(index, 0, round)
        current = index
    } else {
        let player = round % players
        let index = (current - 7) 
        if(index < 0) index += marbles.length
        scores[player] += BigInt(round + marbles[index])
        marbles.splice(index, 1)
        current = index
    }
    update(round)
}

// console.log(scores)
// console.log(Math.max(...scores))
// let max = scores.reduce(function(max, score) {
//     return (max > score) ? max : score
// }, 0)
let max = scores.reduce(function(max, score) {
    return (max > score) ? max : score
}, BigInt(0))
console.log(max)

let end = new Date().getTime() 
console.log('milliseconds:', end - start)

function update(round) {
    let execution_time =new Date().getTime() - start  
    let percent= round / rounds * 100
    let message = ""
    message += "round: " + round 
    message += "\n"
    message += "percent: " + Math.round(percent)
    message += "\n"
    message += "execution time: " + Math.round(execution_time / 1000)
    message += "\n"
    message += "estimated finish: " + Math.round(100 / percent * execution_time / 1000)
    message += "\n"
    process.stdout.moveCursor(0, -4);
    process.stdout.clearScreenDown();
    process.stdout.write(message);
}