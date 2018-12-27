const fs = require('fs') 
// const Tree = require('./tree')

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split(' ').map(v => Number(v))
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split(' ').map(v => Number(v))

let answer = sumMetaEntires(input)
console.log(answer)

function sumMetaEntires(input) {
    const children = input.shift() 
    const metadata = input.shift() 

    let sum = 0 
    for(let i = 0; i < children; i++) {
        sum += sumMetaEntires(input) 
    }

    for(let i = 0; i < metadata; i++) {
        sum += input.shift()
    }

    return sum
}
