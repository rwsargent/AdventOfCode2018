const fs = require('fs') 

let input = fs.readFileSync('./input.txt', 'utf8').split('\n')

let answer = input.reduce(function(sum, val) {
    return sum += parseInt(val)
}, 0)

console.log(answer)