const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').trim()
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').trim()

function reducePoly(input) {
    input = input.split('').reduce(function(acc, curr) {
        if(acc.length === 0) return [curr]
        let prev = acc.pop()
        let diff = Math.abs(prev.charCodeAt(0) - curr.charCodeAt(0))
        if( diff === 32 ) {
            found_pairs = true
            return acc
        }
        acc.push(prev, curr)
        return acc
    }, [])

    let answer = input.join('')
    return answer
}

let answer = reducePoly(input)

let chars = new Array(26).fill().map((v, i) => String.fromCharCode('a'.charCodeAt(0) + i))
let length = {}
for(let c of chars) {
    length[c] = reducePoly(answer.replace(new RegExp(c, 'gi'), '')).length
}
let min = 999999999
let poly = ''
for(let c in length) {
    if(length[c] < min) {
        min = length[c]
        poly = c
    }
}
console.log(poly, min)
return