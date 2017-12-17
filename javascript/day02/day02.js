var tests = require('./tests.js');

function rowdiff(input) {
	var max = input.reduce(function(a, b) {
		return Math.max(a, b);
	})

	var min = input.reduce(function(a, b) {
		return Math.min(a, b);
	})

	return max - min;	
}

function checksum(input) {
	var checksum = 0;
	input.forEach(function(row) {
		checksum += rowdiff(row);
	})

	return checksum;
}

tests.partOne.forEach(function(test) {
	console.log('output', checksum(test.input));
})



function rowEvenDividers(input) {
	var result = 0;

	input.forEach(function(a, i) {
		input.forEach(function(b, j) {
			if (i != j && a % b == 0) {
				result = a / b;
			}
		})
	})

	return result;
}

function checksum2(input) {
	var checksum = 0;
	input.forEach(function(row) {
		checksum += rowEvenDividers(row);
	})

	return checksum;
}

tests.partTwo.forEach(function(test) {
	console.log('output', checksum2(test.input));
})