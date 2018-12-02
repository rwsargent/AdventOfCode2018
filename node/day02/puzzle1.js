const fs = require('fs')

let input = fs.readFileSync('./input.txt', 'utf8').split('\n') 

function getLetterFreq(id) {
    output = {}
    id = id.split('')
    for(var i = 0; i < id.length; i++) {
        var letter = id[i]
        if(!(letter in output)) output[letter] = 0 
        output[letter]++ 
    }
    return output
}


function letterFreqCount(letterCount, count) {
    for(var letter in letterCount) {
        if(letterCount[letter] == count) return true 
    }
    return false
}

let twice = 0 
let thrice = 0 
for(var index in input) {
    var letterFreq = getLetterFreq(input[index]) 
    if(letterFreqCount(letterFreq, 2)) twice++
    if(letterFreqCount(letterFreq, 3)) thrice++
}

let checksum = twice * thrice

// console.log('twice', twice, 'thrice', thrice)
console.log('checksum', checksum)