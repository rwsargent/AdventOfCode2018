const fs = require('fs')

let input = fs.readFileSync('./input.txt', 'utf8').split('\n') 

find_ids: // lets us exit nested loops
for(var a = 0; a < input.length; a++) {
    for(var b = 0; b < input.length; b++) {
        if(b < a || b === a) continue // skip repeat comparisons

        var id1 = input[a].split('')
        var id2 = input[b].split('')

        let distance = id1.filter(function(letter, i) {
            return (id2[i] !== letter)
        }, 0).length

        // we found the one boys
        if(distance === 1) break find_ids
    }
}

let similarLetters = id1.filter(function(letter) {
    return id2.includes(letter)
}).join('')

console.log(id1.join(''), id2.join(''))
console.log(similarLetters)


/* ------------- */
// recursive levenshtien takes too long!!!
/* ------------- */
/*
function levenshtien(s, t) {
    console.log(s, t)
    if(s.length === 0) return t.length 
    if(t.length === 0) return s.length

    return Math.min(
        levenshtien(s.substr(1), t) + 1,
        levenshtien(t.substr(1), s) + 1,
        levenshtien(s.substr(1), t.substr(1)) + (s[0] !== t[0] ? 1 : 0),
    ) + 1;
}

find_ids:
for(var a in input) {
    for(var b in input) {
        if(b < a || b === a) continue
        var distance = levenshtien(input[a], input[b]) - input[a].length
        console.log(distance, input[a], input[b])
        if(distance === 1) break find_ids
    }
}

let id1 = input[a].split('')
let id2 = input[b].split('')
let similarLetters = id1.filter(function(letter) {
    return id2.includes(letter)
}).join('')

console.log(similarLetters)
//*/