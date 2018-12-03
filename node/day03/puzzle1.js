const fs = require('fs') 

let input = fs.readFileSync('./input.txt', 'utf8').split('\n') 

let max_x = 0
let max_y = 0
let claims = input.map(function(val) {
    let pattern = /#(\d+).*?(\d+),(\d+):.*?(\d+)x(\d+)/g
    let matches = pattern.exec(val)
    let row = {
        id: matches['1'],
        x: parseInt(matches[2]),
        y: parseInt(matches[3]),
        w: parseInt(matches[4]),
        h: parseInt(matches[5]),
    }

    let x = row.x + row.w
    if(x > max_x) {
        max_x = x
        console.log(max_x, max_y)
    }
    let y = row.y + row.h
    if(y > max_y) {
        max_y = y
        console.log(max_x, max_y)
    }

    return row 
})

console.log('i', input[0], 'x', max_x, 'y', max_y)

let fabric = new Array(max_x).fill( new Array(max_y).fill('.') )
for(let claim of claims) {
    for(var x = claim.x; x < claim.x + claim.w; x++) {
        for(var y = claim.y; y < claim.y + claim.h; y++) {
            if(fabric[x][y] === '.') {
                fabric[x][y] = claim.id
            } else {
                fabric[x][y] = 'x'
            }
        }
    }
}

let answer = fabric.reduce(function(total, val) {
    return total += val.reduce(function(row_sum, val) {
        if(val === 'x') row_sum++
        return row_sum
    }, 0) 
}, 0)

console.log(answer)