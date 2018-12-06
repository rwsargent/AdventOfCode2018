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

// transform entries into 
// {
//     [guard_id]: [ times asleep by minute ],
//     10: [0, 1, 1, 2, 3, etc. ],
// }
let guards = {}
var id, status, minute, prev_minute;
for(let entry of entries) {
    if(entry.status.includes('Guard')) {
        id = entry.status.match(/#\d+/)[0].replace('#', '')
        if(!(id in guards)) {
            guards[id] = new Array(60).fill(null).map(v => 0) 
        }

        prev_status = 'awake'
        status = 'awake' 
        prev_minute = 0
        minute = entry.date.getMinutes()
    } else if(entry.status.includes('falls asleep')) {
        prev_status = status
        status = 'asleep' 
        prev_minute = minute
        minute = entry.date.getMinutes()
    } else if(entry.status.includes('wakes up')) {
        prev_status = status
        status = 'awake' 
        prev_minute = minute
        minute = entry.date.getMinutes()
    } else {
        // nothing changes
    }

    for(var i = prev_minute; i < minute; i++) {
        if(prev_status == 'asleep') guards[id][i]++
    }
}

// find the guard with the most time asleep and their max minute spent sleeping
let best = {
    id: 0,
    total_sleep: 0,
    minute_index: 0,
    minute_sleep: 0,
}
for(let id in guards) {
    let guard = {
        id: id,
        total_sleep: 0,
        minute_index: 0,
        minute_sleep: 0,
    }
    guard.total_sleep = guards[id].reduce(function(total, minute_sleep, minute_index) {
        if(minute_sleep > guard.minute_sleep) {
            guard.minute_sleep = minute_sleep 
            guard.minute_index = minute_index
        }
        return total += minute_sleep 
    }, 0)

    if(guard.minute_sleep > best.minute_sleep) {
        best = guard
    }
}

let answer = best.id * best.minute_index
console.log(answer)