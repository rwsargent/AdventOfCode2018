const fs = require('fs') 
// const Tree = require('./tree')

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split(' ').map(v => Number(v))
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split(' ').map(v => Number(v))

let answer = sillySum(input)
console.log(answer)

function sillySum(input) {
    const children = input.shift() 
    const metadata = input.shift() 

    let sum = 0

    if(!children) {
        for(let i = 0; i < metadata; i++) {
            sum += input.shift()
        }
    } else {
        let child = [] 
        for(let i = 0; i < children; i++) {
            child.push(sillySum(input))
        }

        let metas = []
        for(let i = 0; i < metadata; i++) {
            metas.push(input.shift())
        }

        for(let index of metas) {
            let i = index - 1 // because arrays start at zero
            if(i >= 0 && i < child.length) sum += child[i]
        }
    }

    return sum
}
