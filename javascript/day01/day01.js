var tests = require('./tests.js');

function captcha (input) {
	var sum = 0;
	var num_array = input.split('').map(function(v) {
		return parseFloat(v);
	});

	num_array.forEach(function(num, index)  {
		var compare_index = (index === num_array.length - 1) ? 0 : index + 1;

		if (num === num_array[compare_index]) {
			sum += num;
		}
	});

	return sum;
}

tests.partOne.forEach(function(test) {
	console.log('testing', test.input);
	console.log('output', captcha(test.input));
})

//-------------------------------
// Part Two
//-------------------------------


function captcha2(input) {
	var sum = 0;
	var num_array = input.split('').map(function(v) {
		return parseFloat(v);
	})

	num_array.forEach(function(num, index) {
		// var compare_index = (index === (num_array.length - 1) / 2) ? 0 : index + 1;
		var offset = num_array.length / 2;
		compare_index = index + offset;
		if(compare_index > (num_array.length - 1)) {
			compare_index = compare_index % num_array.length;
		}

		if(num === num_array[compare_index]) {
			sum += num;
		}
	})

	return sum;
}

tests.partTwo.forEach(function(test) {
	console.log('testing', test.input);
	console.log('output', captcha2(test.input));
})