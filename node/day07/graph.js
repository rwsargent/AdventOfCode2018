function Graph() {
    this.vertexs = [] 
    this.edges = [] 
    this.reqs = []
    this.relations = 0
}

Graph.prototype.addVertex = function(vertex) {
    this.vertexs.push(vertex) 
    this.edges[vertex] = [] 
    this.reqs[vertex] = [] 
}

Graph.prototype.addEdge = function(vertex1, vertex2) {
    this.edges[vertex1].push(vertex2) 
    this.reqs[vertex2].push(vertex1)
    this.relations++
}

Graph.prototype.print = function(type) {
    type = type || 'edges'
    let vertexs = this.vertexs 
    vertexs.sort()
    let print = '\n' + type + '\n'
    print += vertexs.map(function(vertex) {
        return vertex + ' -> ' + this[type][vertex].join(', ')
    }.bind(this)).join('\n')
    return print
}

Graph.prototype.topologicalSort = function() {
    let visited = [] 

    for(let vertex of this.vertexs) {
        if(visited.includes(vertex)) continue 
        helper(vertex, visited, this)
    }

    return visited.reverse()

    function helper(vertex, visited, graph) {
        let edges = graph.edges[vertex] 
        edges.sort().reverse()
        for(let edge of edges) {
            if(edge === vertex) continue
            if(visited.includes(edge)) continue
            helper(edge, visited, graph)
        }
        visited.push(vertex) 
    }
}

Graph.prototype.traverse = function(vertex) {
    let visited = [] 
    let queue = [] 

    queue.push(vertex) 

    while(queue.length) {
        queue.sort()
        vertex = queue.shift() 
        if(!this.edges[vertex].length && !queue.length) {
            visited.push(vertex) 
        }
        if(!this.edges[vertex].length) continue
        if(!visited.includes(vertex)) visited.push(vertex)
        let edges = this.edges[vertex]
        for(let edge of edges) {
            if(visited.includes(edge)) continue 
            queue.push(edge)
        }
    }
    return visited
}

module.exports = Graph