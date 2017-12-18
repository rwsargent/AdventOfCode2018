var tests = require('./tests.js');

function dfs(input, target, set, uniques) {
	var target = (target) ? target : 0;
	var set = (set) ? set : { 0: true };

	input[target].forEach(function(value) {
		// console.log(target, set, value);
		if(!set[value]) {
			set[value] = true;
			set = dfs(input, value, set);
		}
	})

	return set;
}

function groups(input) {
	return Object.keys(dfs(input)).length;
}


console.log(groups(tests[0].input));
console.log(groups(tests[1].input));
console.log(groups(tests[2].input));

function numberOfGroups(input) {
	var groups = 0;
	var set = {};

	Object.keys(input).forEach(function(key) {
		if(!set[key]) {
			groups++;
			set = dfs(input, key, set);
		}
	})

	return groups;
}

console.log(numberOfGroups(tests[0].input));
console.log(numberOfGroups(tests[1].input));
console.log(numberOfGroups(tests[2].input));