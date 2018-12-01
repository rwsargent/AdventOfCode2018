// bottom right is always odd squared number
// 3 * 3 = 9
// 5 * 5 = 25
//
// etc.
// 
//  37  36  35  34  33  32  31
//  38  17  16  15  14  13  30
//  39  18   5   4   3  12  29
//  40  19   6   1   2  11  28
//  41  20   7   8   9  10  27
//  42  21  22  23  24  25  26
//  43  44  45  46  47  48  49
//
//
// (25 - x) mod 4
// 0  1  2  3  0
// 3           1
// 2           2
// 1           3
// 0  1  2  3  0
//
//  mod 7
//  2  1  0  6  5  4  3
//  3                 2
//  4                 1
//  5                 0
//  6                 6
//  0                 5
//  1  2  3  4  5  6  0
//
//  (49 - x) mod 6
//  0  1  2  3  4  5  0
//  5                 1
//  4                 2
//  3                 3
//  2                 4
//  1                 5
//  0  1  2  3  4  5  0
//
//  12 11 10 9  8  7  6
//  11                5
//  10                4
//  9                 3
//  8                 2
//  7                 1
//  6  5  4  3  2  1  0
//
//  3  2  1  0  1  2  3
//  2                 2
//  1                 1
//  0                 0
//  1                 1
//  2                 2
//  3  2  1  0  1  2  3
//
//

function manhattanDistance(input) {
	var boxWidth = getBoxWidth(input);
	var stepsToCenter = Math.floor(boxWidth / 2);
	var stepsFromCorner = Math.abs((boxWidth * boxWidth - input) % (boxWidth - 1) - stepsToCenter);

	return stepsToCenter + stepsFromCorner;
}

function getBoxWidth(input) {
	var base = Math.ceil(Math.sqrt(input));
	return ( base % 2 == 0 ) ? base + 1 : base;
}


var tests = [
	{ input: 10, expects: 3 },
	{ input: 11, expects: 2 },
	{ input: 12, expects: 3 },
	{ input: 13, expects: 4 },
	{ input: 14, expects: 3 },
	{ input: 15, expects: 2 },
	{ input: 16, expects: 3 },
	{ input: 17, expects: 4 },
	{ input: 18, expects: 3 },
	{ input: 19, expects: 2 },
	{ input: 20, expects: 3 },
	{ input: 21, expects: 4 },
	{ input: 22, expects: 3 },
	{ input: 23, expects: 2 },
	{ input: 24, expects: 3 },
	{ input: 25, expects: 4 },
	{ input: 26, expects: 5 },
	{ input: 27, expects: 4 },
	{ input: 28, expects: 3 },
	{ input: 29, expects: 4 },
	{ input: 30, expects: 5 },
	{ input: 31, expects: 6 },
	{ input: 32, expects: 5 },
	{ input: 33, expects: 4 },
	{ input: 34, expects: 3 },
	{ input: 35, expects: 4 },
	{ input: 36, expects: 5 },
	{ input: 37, expects: 6 },
	{ input: 38, expects: 5 },
	{ input: 39, expects: 4 },
	{ input: 40, expects: 3 },
	{ input: 41, expects: 4 },
	{ input: 42, expects: 5 },
	{ input: 43, expects: 6 },
	{ input: 44, expects: 5 },
	{ input: 277678, expects: 5 }
];

tests.forEach(function(test) {
	var result = manhattanDistance(test.input);
	if( result == test.expects ) {
		console.log('test input:', test.input, 'passed');	
	} else {
		console.log('test input:', test.input, 'failed', 'output:', result);	
	}
});