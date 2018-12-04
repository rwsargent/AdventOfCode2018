const fs = require('fs') 

// let input = fs.readFileSync(__dirname + '/test.txt', 'utf8').split('\n') 
let input = fs.readFileSync(__dirname + '/input.txt', 'utf8').split('\n') 

// transform data
let entries = input.map(function(val) {
    let pattern = /\[(.*?)]\s+(.*)/g
    let matches = pattern.exec(val)
    let entry = {
        date: new Date(matches[1]),
        status: matches[2],
    }
    return entry
})

// sort by date
entries.sort(function(a, b) {
    return a.date - b.date
})

console.log(entries.slice(0, 10))