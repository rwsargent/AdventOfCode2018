tests = [
	'1122',
	'1111',
	'1234',
	'91212129',
]

def captcha(word) :
	sum = 0

	for index in range(len(word)) :
		if( index == len(word) - 1 ) :
			compare_index = 0
		else :
			compare_index = index + 1

		if( word[index] == word[compare_index] ) :
			sum += int(word[index])

	return sum;

for t_index in range(len(tests)) :
	print('testing', tests[t_index])
	print('output', captcha(tests[t_index]))