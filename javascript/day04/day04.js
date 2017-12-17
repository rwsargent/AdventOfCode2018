var tests = require('./tests.js');

function checkPassword(password) {
	var valid = true;
	var words = password.split(' ');
	var something = {};

	// bigO notation of n
	words.every(function(word) {
		if( word in something ) {
			valid = false;
			return false;
		}

		something[word] = true;
		return true;
	});

	// bigO notation of n^2
	// words.every(function(word) {
	// 	if(words.filter(function(newword) {
	// 		return (word === newword);
	// 	}).length > 1) {
	// 		valid = false;
	// 		return false;
	// 	}
	// });

	return valid;
}

tests.partOne.forEach(function(test) {
	console.log('test input', test.input, 'is: ', checkPassword(test.input))
	console.log((checkPassword(test.input) === test.expects) ? 'test passed' : 'test failed');
})