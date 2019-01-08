const fs = require('fs') 

let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
// let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let state = /initial state: (.*)/.exec(input.shift()).slice(1, 2).shift()
console.log(state)
input.shift()
let patterns = input.map(function(p) {
    let matches = /([\.#]+) => ([\.#])/.exec(p) 
    return matches[1]
})
console.log(patterns)

let r = new_generation(state)
console.log(r)

function new_generation(state) {
    state = '...' + state
    state = state.split('')

    let new_state = state.splice(0, 5).join('')
    let conv = new_state
    return conv

}