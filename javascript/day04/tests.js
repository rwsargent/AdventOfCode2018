var tests = {
	partOne: [
		{
			input: 'aa bb cc dd ee',
			expects: true
		},
		{
			input: 'aa bb cc dd aa',
			expects: false
		},
		{
			input: 'aa bb cc dd aaa',
			expects: true
		}
	]
};

module.exports = tests;