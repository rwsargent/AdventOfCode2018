const fs = require('fs') 

let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
// let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')


//-------------------
function transformInput(input) {
    let steps = input.map(function(step) {
        let matches = step.match(/[A-Z]/g)
        console.log(matches)
        return {
            data: matches[1],
            child: matches[2],
        }
    })
    console.log(steps)
    return steps
}

function Node(data) {
    this.data = data
    this.parent = null 
    this.children = []
}

function Tree(data) {
    let node = new Node(data) 
    this._root = node
}

Tree.prototype.depthFirst = function(callback) {
    (function recurse(currentNode) {
        for(let node of currentNode.children) {
            recurse(node)
        }
        callback(currentNode)
    })(this._root)
}

Tree.prototype.breadthFirst = function(callback) {
    let queue = []
    queue.push(this._root) 
    currentNode = queue.shift() 

    while(currentNode) {
        for(let node of currentNode.children) {
            queue.push(node)
        }
        callback(currentNode) 
        currentNode = queue.shift()
    }
} 

Tree.prototype.contains = function(callback, traversal) {
    traversal.call(this, callback)
}

Tree.prototype.add = function(data, toData, traversal) {
    let child = new Node(data) 
    let parent = null 
    let callback = function(node) {
        if(node.data === toData) {
            parent = node
        }
    }

    this.contains(callback, traversal) 

    if(parent) {
        parent.children.push(child) 
    } else {
        throw new Error('Cannot add node to a non-existant parent')
    }
}

Tree.prototype.remove = function(data, fromData, traversal) {
    let tree = this 
    let parent = null 
    let childToRemove = null 
    let index 

    let callback = function(node) {
        if(node.data === fromData) {
            parent = node
        }
    }

    this.contains(callback, traversal) 

    if(parent) {
        index = findIndex(parent.children, data) 

        if(index === undefined) {
            throw new Error('Node to remove does not exist.') 
        } else {
            childToRemove = parent.children.splice(index, 1)
        }
    } else {
        throw new Error('Parent does not exist.')
    }

    return childToRemove

}

function findIndex(arr, data) {
    for(let i = 0; i < arr.length; i++) {
        if(arr.data === data) return i
    }
}

function Queue() {
    this.first = null 
    this.size = 0
}

Queue.prototype.enqueue = function(data) {
    let node = new Node(data) 
    if(!this.first) {
        this.first = node 
    } else {
        n = this.first 
        while(n.next) {
            n = n.next 
        }
        n.next = node
    }
    this.size++
    return node
}

Queue.prototype.dequeue = function(data) {
    let temp = this.first 
    this.first = this.first.next 
    this.size-- 
    return temp
}




let steps = transformInput(input)
let tree = new Tree(steps[0].data)
for(let step of steps) {
    tree.add(step.child, step.data, tree.breadthFirst) 
}
console.log(tree)