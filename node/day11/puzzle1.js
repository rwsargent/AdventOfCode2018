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
grid.forEach(function(x, y) {
    x.forEach(function(cell, x) {
        let power = getPowerConvolution(x, y, grid)
        if(power > max) {
            max = power
            coord = [x - 1, y - 1]
        }
    })
})
console.log(max, coord)

function getPower(x, y, serial) {
    let rack_id = x + 10 
    let power_level = rack_id * y 
    power_level += serial 
    power_level *= rack_id 
    power_level = Math.floor(power_level / 100) % 10
    power_level -= 5
    return power_level
}

function getPowerConvolution(x, y, grid) {
    let power = 0
    if(x < 2 || y < 2) return power

    for(let yy = y; yy > y - 3; yy--) {
        for(let xx = x; xx > x - 3; xx--) {
            power += grid[yy][xx]
        }
    }
    return power
}
