function Graph() {
    this.vertexs = [] 
    this.edges = [] 
    this.relations = 0
}

Graph.prototype.addVertex = function(vertex) {
    this.vertexs.push(vertex) 
    this.edges[vertex] = [] 
}

Graph.prototype.addEdge = function(vertex1, vertex2) {
    this.edges[vertex1].push(vertex2) 
    this.edges[vertex2].push(vertex1) 
    this.relations++
}

Graph.prototype.print = function() {
    let print = this.vertexs.map(function(vertex) {
        return vertex + ' -> ' + this.edges[vertex].join(', ')
    }.bind(this)).join('\n')
    return print
}

Graph.prototype.traverse = function(vertex, callback) {
    let queue = [] 
    let visited = [] 
    let order = []

    queue.push(vertex) 
    visited.push(vertex)

    while(queue.length) {
        queue.sort()
        vertex = queue.shift() 
        visited.push(vertex)
        let edges = this.edges[vertex]
        for(let edge of edges) {
            if(visited.includes(edge)) continue 
            queue.push(edge)
        }
    }
    return visited
}

module.exports = Graph