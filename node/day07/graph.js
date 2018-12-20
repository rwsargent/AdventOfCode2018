function Graph() {
    this.nodes = [] 
    this.edges = [] 
}

Graph.prototype.addNode = function(node) {
    if(this.nodes.includes(node)) return 
    this.nodes.push(node) 
    this.edges[node] = [] 
}

Graph.prototype.addEdge = function(node, edge) {
    if(this.edges[node].includes(edge)) return 
    this.edges[node].push(edge) 
}

Graph.prototype.create = function(input) {
    return input
}

Graph.prototype.print = function(type) {
    type = type || 'edges'
    let nodes = this.nodes 
    nodes.sort()
    let print = '\n' + type + '\n'
    print += nodes.map(function(node) {
        return node + ' -> ' + this[type][node].join(', ')
    }.bind(this)).join('\n')
    return print
}

Graph.prototype.dfsOrder = function() {
    let visited = [] 
    let nodes = this.nodes
    nodes.sort().reverse()

    for(let node of nodes) {
        if(visited.includes(node)) continue 
        helper(node, visited, this)
    }

    return visited.reverse()

    function helper(node, visited, graph) {
        let edges = graph.edges[node] 
        edges.sort().reverse()
        for(let edge of edges) {
            if(visited.includes(edge)) continue
            helper(edge, visited, graph)
        }
        visited.push(node) 
    }
}

Graph.prototype.kahnOrder = function() {
    let order = [] 
    let nodes = this.nodes.sort()
    let edges = this.edges
    let parents = nodes.filter(function(node) {
        return !edges[node].length
    }).sort()

    while(parents.length) {
        let next = parents.shift() 
        order.push(next)

        delete(edges[next]) 

        for(let node in edges) {
            edges[node] = edges[node].filter(function(edge) {
                return edge != next
            })
            if(!edges[node].length && !parents.includes(node)) parents.push(node)
        }

        parents.sort()
    }

    return order
}

module.exports = Graph