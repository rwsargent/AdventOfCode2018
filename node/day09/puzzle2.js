let start = new Date().getTime()
let players = 418
let rounds = 71339 * 100

function LinkedList(value) {
    let node = new Node(value)
    node.next = node 
    node.prev = node
    this.current = node
}

function Node(value) {
    this.value = value 
    this.prev = null 
    this.next = null 
}

LinkedList.prototype.append = function(value) {
    let node = new Node(value) 

    node.next = this.current.next
    node.prev = this.current
    this.current.next = node
    node.next.prev = node

    this.current = node
    return this
}

LinkedList.prototype.next = function(steps) {
    steps = steps || 1
    while(steps > 0) {
        this.current = this.current.next
        steps--
    }
    return this
}

LinkedList.prototype.prev = function(steps) {
    steps = steps || 1
    while(steps > 0) {
        this.current = this.current.prev
        steps--
    }
    return this
}

LinkedList.prototype.remove = function() {
    let node = this.current 
    node.next.prev = node.prev
    node.prev.next = node.next
    this.current = node.next 
    delete(node)
    return this
}

let scores = new Array(players).fill(null).map(s => 0)
let marbles = new LinkedList(0)

for(let round = 1; round <= rounds; round++) {
    if(round % 23) {
        if(round > 1) marbles.next()
        marbles.append(round)
    } else {
        let player = round % players
        let marble = marbles.prev(7).current.value
        marbles.remove()
        scores[player] += round + marble
    }
}

let max = scores.reduce(function(max, score) {
    return (max > score) ? max : score
}, 0)
console.log(max)

let end = new Date().getTime() 
console.log('milliseconds:', end - start)