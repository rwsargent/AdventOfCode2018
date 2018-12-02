const fs = require('fs') 

let input = fs.readFileSync('./input.txt', 'utf8').split('\n') 

let frequency = 0
let frequencies = [] 
let answer = false
let index = 0
while(answer === false) {
    let val = input[index % input.length]
    frequency += parseInt(val)
    answer = frequencies.includes(frequency)
    frequencies.push(frequency)
    index++;
}

console.log(frequency)