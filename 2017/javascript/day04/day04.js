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

function validPasswords(passwords) {
	var count = 0;
	passwords.forEach(function(password) {
		(checkPassword(password)) ? count++ : null;
	})
	return count;
}

tests.tests.partOne.forEach(function(test) {
	console.log('test input', test.input, 'is: ', checkPassword(test.input))
	console.log((checkPassword(test.input) === test.expects) ? 'test passed' : 'test failed');
})

console.log(validPasswords(tests.passwords));

function checkAnagrams(password) {
	var valid = true;
	var words = password.split(' ').map(function(word) {
		return word.split('').sort().join('');
	});
	var set = {};

	// bigO notation of n
	words.every(function(word) {
		if( word in set ) {
			valid = false;
			return false;
		}

		set[word] = true;
		return true;
	});

	return valid;
}

function validAnagrams(passwords) {
	var count = 0;
	passwords.forEach(function(password) {
		(checkAnagrams(password)) ? count++ : null;
	})
	return count;
}

tests.tests.partOne.forEach(function(test) {
	console.log('test input', test.input, 'is: ', checkAnagrams(test.input))
	console.log((checkAnagrams(test.input) === test.expects) ? 'test passed' : 'test failed');
})

console.log(validAnagrams(tests.passwords));