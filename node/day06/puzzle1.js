const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n')
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n')

let {coords, grid, width, height, distance} = transformData(input)

for(let coord of coords) {
    for(let dist = 0; dist <= distance; dist++) {
        let new_claim = true
        let mcoords = getManhattanCoords(coord, dist)
        for(let mcoord of mcoords) {
            // skip coordinates outside bounds
            if(mcoord.x < 0 
                || mcoord.y < 0
                || mcoord.x > width - 1 
                || mcoord.y > height - 1
            ) continue;

            let gcoord = grid[mcoord.y][mcoord.x]

            // if there is a coordinate closer it 'claims' the area
            if(dist < gcoord.distance) {
                gcoord.claim = coord.id
                gcoord.distance = dist
                new_claim = true 
            } else if(gcoord.distance == dist) {
                gcoord.claim = 0
                new_claim = true
            }
        }
        if(new_claim == false) break;
    }
}

// let g = visualizeGrid(grid)
// console.log(g)

let ids = {}
for(let y = 0; y < grid.length; y++) {
    for(let x = 0; x < grid[y].length; x++) {
        let coord = grid[y][x]

        if(!(coord.claim in ids)) {
            ids[coord.claim] = {
                area: 0,
                infinite: false,
            }
        }

        ids[coord.claim].area++
        if(x === 0
            || y === 0
            || x === width - 1 
            || y === height - 1
        ) {
            ids[coord.claim].infinite = true
        }
    }
}

let max_area = 0
let max_id = 0
for(let id in ids) {
    let claim = ids[id]
    if(claim.infinite) continue
    if(claim.area > max_area) {
        max_area = claim.area 
        max_id = id 
    }
}
console.log(max_id, max_area)


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
            area: 0,
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
        .map(function(v) { return { claim: 0, distance: distance } }))

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