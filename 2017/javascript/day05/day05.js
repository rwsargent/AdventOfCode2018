var runTests = require('./tests.js');

function countJumpSteps(arr) {
	var index = 0;
	var steps = 0;
	var newindex;

	while(arr[index] !== undefined) {
		newindex = arr[index] + index;
		arr[index] += 1;
		steps++;
		index = newindex;
	}

	return steps;
}

function p2Jumps(arr) {
	var index = 0;
	var steps = 0;
	var newindex;

	while(arr[index] !== undefined) {
		newindex = arr[index] + index;
		(arr[index] >= 3) ? arr[index] -= 1 : arr[index] += 1;
		steps++;
		index = newindex;
	}

	return steps;
}

runTests(countJumpSteps);
runTests(p2Jumps);