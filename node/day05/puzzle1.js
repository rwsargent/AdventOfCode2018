const fs = require('fs') 

let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').trim()
// let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').trim()
input = input.split('')

input = input.reduce(function(acc, curr) {
    if(acc.length === 0) return [curr]
    let prev = acc.pop()
    let diff = Math.abs(prev.charCodeAt(0) - curr.charCodeAt(0))
    if( diff === 32 ) {
        console.log('removing: ', prev, curr)
        found_pairs = true
        return acc
    }
    acc.push(prev, curr)
    return acc
}, [])

let answer = input.join('')
console.log(answer)
console.log(input.length)