const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n') 
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n') 

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
    }
    let y = row.y + row.h
    if(y > max_y) {
        max_y = y
    }

    return row 
})
console.log('first claim:', input[0], 'x:', max_x, 'y:', max_y)

let fabric = new Array(max_x).fill(null).map(v => new Array(max_y).fill(null).map(v => '.'))
let ids = {}
for(let claim of claims) {
    ids[claim.id] = true
    for(var x = claim.x; x < claim.x + claim.w; x++) {
        for(var y = claim.y; y < claim.y + claim.h; y++) {
            if(fabric[x][y] === '.') {
                fabric[x][y] = claim.id
            } else {
                delete ids[claim.id] // eh this happens alot who cares
                if(fabric[x][y] !== 'x') delete ids[fabric[x][y]] 
                fabric[x][y] = 'x'
            }
        }
    }
}
console.log(ids)
return 

let answer = fabric.reduce(function(total, x) {
    return total += x.reduce(function(row_sum, y) {
        if(y === 'x') row_sum++
        return row_sum
    }, 0) 
}, 0)
console.log(answer)
// console.log(fabric)