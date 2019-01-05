const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let data = input.map(function(line) {
    let matches = /position.*?<(.*?)>.*velocity.*?<(.*?)>/g.exec(line)
    let position = matches[1].split(',').map(n => Number(n))
    let velocity = matches[2].split(',').map(n => Number(n))

    return {
        position: position,
        velocity: velocity,
    }
})

function getMaxBounds(data) {
    let max = [0, 0]
    let min = [0, 0]

    for(set of data) {
        let x = set.position[0] 
        let y = set.position[1] 

        if(x > max[0]) max[0] = x 
        if(y > max[1]) max[1] = y

        if(x < min[0]) min[0] = x
        if(y < min[1]) min[1] = y
    }

    min = min.map(m => m++) 
    max = max.map(m => m++) 

    return [].concat(min, max)
}

function getBoundsSize(bounds) {
    let size = (Math.abs(bounds[0]) + Math.abs(bounds[2])) * (Math.abs(bounds[1]) + Math.abs(bounds[3]))
    return size
}

function getSize(data) {
    let bounds = getMaxBounds(data) 
    let size = getBoundsSize(bounds)
    return size
}

function forwardOneTick(data) {
    for(set of data) {
        set.position[0] += set.velocity[0]
        set.position[1] += set.velocity[1]
    }

    return data
}

function backwardOneTick(data) {
    for(set of data) {
        set.position[0] -= set.velocity[0]
        set.position[1] -= set.velocity[1]
    }

    return data
}

let size = getSize(data)
let old_size = size + 1 
let tick = 0
while(size < old_size) {
    old_size = size
    data = forwardOneTick(data)
    size = getSize(data)
    tick++
    // console.log(size, tick)
}
data = backwardOneTick(data)
tick--
size = getSize(data)


bounds = getMaxBounds(data)

let points = data.map(function(set) {
    return set.position
})

let x = new Array(bounds[2] + 1).fill(null).map(v => ' ')
let y = new Array(bounds[3] + 1).fill(null).map(v => x.slice())

let printable = y.map(function(x, y) {
    return x.map(function(c, x) {
        let hash = points.filter(function(point) {
            let hash = point[0] == x && point[1] == y
            return hash
        }).length
        return (hash) ? '#' : c
    }).join('')
}).join('\n')

console.log(printable)
console.log(size, tick)