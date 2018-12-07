const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let {coords, grid, width, height, distance} = transformData(input)
for(let y = 0; y < grid.length; y++) {
    for(let x = 0; x < grid[y].length; x++) {
        let distance = coords.reduce(function(sum, c) {
            return sum += manhattanDistance({ x: x, y: y }, c)
        }, 0)
        if(distance < 10000) grid[y][x] = 1
    }
}

// console.log(grid)

let answer = grid.reduce(function(acc, y) {
    return acc.concat(y)
}).filter(function(v) {
    return v === 1
}).length
console.log(answer)

//--------------------
// Helper Functions
//--------------------

// pretty prints a grid with claimed coord ids
function visualizeGrid(grid) {
    let visual = grid.map(function(x) {
        return x.map(function(y) {
            return y.claim
        }).join(' ')
    }).join('\n')
    return visual
}

// returns a list of coordinates and a grid
function transformData(input) {
    let width = 0 
    let height = 0
    let coords = input.map(function(coord, i) {
        let matches = /(\d+).*?(\d+)/.exec(coord)
        // create coordinate obj, we are going to use 0 as 'unmatched'
        let c = {
            id: parseInt(i) + 1,
            x: parseInt(matches[1]),
            y: parseInt(matches[2]),
        }

        // check for new maxes
        if(c.x > width) width = c.x 
        if(c.y > height) height = c.y 

        return c
    })

    width++
    height++
    let distance = width + height
    let grid = new Array(height)
        .fill(null)
        .map(v => new Array(width)
        .fill(null)
        .map(v => 0))

    return {
        coords: coords,
        grid: grid,
        width: width,
        height: height,
        distance: distance,
    }
}

// get a list of coordinates that are x manhattan distance from a coordinate
function getManhattanCoords(coord, distance) {
    let coords = [] 
    for(let x = -distance; x <= distance; x++) {
        for(let y = -distance; y <= distance; y++) {
            if(Math.abs(x) + Math.abs(y) !== distance) continue;
            coords.push({ x: coord.x + x, y: coord.y + y })
        }
    }
    return coords
}

function manhattanDistance(a, b) {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y)
}