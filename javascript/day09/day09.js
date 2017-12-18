function countGroups(input) {
	var stack = [];
	var count = 0;

	var noEscapes = clearEscapes(input);
	var noGarbage = clearGarbage(noEscapes);
	var onlyGroups = noGarbage.replace(/,/g, '').split('');	

	onlyGroups.forEach(function(char) {
		if(char == '{') {
			stack.push(char);
		}

		if(char == '}') {
			count += stack.length; 
			stack.pop();
		}
	})

	return count;
}

function clearEscapes(input) {
	return input.replace(/(!.{1})/gi, '');
}

function clearGarbage(input) {
	return input.replace(/<.*?>/gi, '');
}

var escapes = require('./tests.js').escapes;
var tests = require('./tests.js').tests;

tests.forEach(function(test, i) {
	var count = countGroups(test.input);
	console.log('input', test.input);
	console.log('count', count);
	console.log((count === test.expects) ? 'test passed' : 'test failed')
	if(i == 11 ) {
		console.log(countGarbage(test.input))
	}
	console.log('--------------------')
});

function countGarbage(input) {
	var noEscapes = clearEscapes(input);

	// get all matches
	// get length for each match minus '<' and '>' chars
	// sum all lengths
	var sum = noEscapes
		.match(/<(.*?)>/g) 
		.map( m => m.length - 2 ) 
		.reduce(function(a, b) { 
			return a + b
		})
	return sum;
}