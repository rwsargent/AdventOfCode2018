const fs = require('fs') 

function Node(value, index) {
    this.value = value 
    this.index = index
    this.prev = null 
    this.next = null
}

function LinkedList(value) {
    let index = 0
    let node = new Node(value, index)
    node.next = null 
    node.prev = null 
    this._head = node 
    this._tail = node
    return this
}

LinkedList.prototype.append = function(value) {
    let index = this._tail.index + 1
    let node = new Node(value, index)
    node.prev = this._tail 
    node.next = null 
    this._tail = node 
    return this
}

LinkedList.prototype.prepend = function(value) {
    let index = this._head.index - 1 
    let node = new Node(value, index) 
    node.next = this._head 
    node.prev = null 
    this._head = node 
    return this
}

let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
// let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let state = /initial state: (.*)/.exec(input.shift()).slice(1, 2).shift()
let plants = new LinkedList(state[0])
for(let plant of state) {
    plants.append(plant)
}
console.log(state)
let plant = plants._head 
while(plant.next !== null) {
    state = plant.value 
}
console.log(plants)
input.shift()
let patterns = input.map(function(p) {
    let matches = /([\.#]+) => ([\.#])/.exec(p) 
    return matches[1]
})
console.log(patterns)