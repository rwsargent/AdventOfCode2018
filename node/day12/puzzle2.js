let serial = 9445
let size = 300
let x = new Array(size).fill(null).map(v => 0)
let y = new Array(size).fill(null).map(v => x.slice())


let grid = y.map(function(x, y) {
    return x.map(function(power, x) {
        return getPower(x + 1, y + 1, serial)
    })
})

// let result = grid.slice(43, 48).map(function(x, y) {
//     return x.slice(31, 36)
// })
// console.log(result)
// console.log(getPowerConvolution(3, 3, result))
// return

let max = 0 
let coord = [0, 0]
let convo_size = 0
for(size = 3; size < 300; ++size) {
    grid.forEach(function(x, y) {
        x.forEach(function(cell, x) {
            let power = getPowerConvolution(x, y, grid, size)
            if(power > max) {
                max = power
                coord = [x - size + 2, y - size + 2]
                convo_size = size
            }
        })
    })
}
console.log(max, coord, convo_size)

function getPower(x, y, serial) {
    let rack_id = x + 10 
    let power_level = rack_id * y 
    power_level += serial 
    power_level *= rack_id 
    power_level = Math.floor(power_level / 100) % 10
    power_level -= 5
    return power_level
}

function getPowerConvolution(x, y, grid, size) {
    size = size || 3
    let power = 0
    if(x < size - 1 || y < size - 1) return power

    for(let yy = y; yy > y - size; yy--) {
        for(let xx = x; xx > x - size; xx--) {
            power += grid[yy][xx]
        }
    }

    return power
}
