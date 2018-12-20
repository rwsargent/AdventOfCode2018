const fs = require('fs') 
const Graph = require('./graph')

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')
let steps = input.map(function(step) {
    let matches = step.match(/[A-Z]/g)
    return {
        edge: matches[1],
        node: matches[2],
    }
})

let graph = new Graph()
for(let step of steps) {
    graph.addNode(step.node) 
    graph.addNode(step.edge) 
    graph.addEdge(step.node, step.edge)
}

console.log(graph)
console.log(graph.kahnOrder().join(''))