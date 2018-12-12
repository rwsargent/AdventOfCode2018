const fs = require('fs') 
const Graph = require('./graph')

let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
// let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let edges = transformInput(input)
let graph = new Graph()
for(let edge of edges) {
    if(!graph.vertexs.includes(edge.v1)) graph.addVertex(edge.v1)
    if(!graph.vertexs.includes(edge.v2)) graph.addVertex(edge.v2)
}
for(let edge of edges) {
    graph.addEdge(edge.v1, edge.v2)
}
console.log(graph.print())
console.log(graph.traverse('C'))

//-------------------
function transformInput(input) {
    let steps = input.map(function(step) {
        let matches = step.match(/[A-Z]/g)
        return {
            v1: matches[1],
            v2: matches[2],
        }
    })
    return steps
}