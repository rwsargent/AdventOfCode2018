package hash

import "fmt"

func KnotHash(input string) [16]byte{
	lengths := inputToLengths(input)

	var bytes [256]byte
	for idx, _ := range bytes {
		bytes[idx] = byte(idx)
	}

	var current, skip = 0, 0

	// round
	for idx := 0; idx < 64; idx++ {
		for _, length := range lengths {
			reverseSublist(&bytes, current, length)
			current = (current + length + skip) % len(bytes)
			skip++
		}
	}

	//densify
	var output [16]byte
	for block := 0; block < 16; block++ {
		var blockValue byte
		for blockIdx := 0; blockIdx < 16; blockIdx++ {
			blockValue ^= bytes[(block*16)+blockIdx]
		}
		output[block] = blockValue
	}

	return output
}

func HashToString(hash [16]byte) string {
	var output = ""
	for _, byte := range hash {
		output += fmt.Sprintf("%02x", byte)
	}
	return output
}

func inputToLengths(input string) []int {
	lengths := make([]int, (len(input) + 5))
	for idx, char := range input {
		lengths[idx] = int(char)
		idx++
	}
	
	lengths[len(input)    ] = 17;
	lengths[len(input) + 1] = 31;
	lengths[len(input) + 2] = 73;
	lengths[len(input) + 3] = 47;
	lengths[len(input) + 4] = 23;

	return lengths
}

func reverseSublist(list *[256]byte, current int, length int) {
	for idx := 0; idx < length / 2; idx++ {
		list[(idx + current) % len(list)], list[((current + length - 1) - idx) % len(list)] = list[((current + length - 1) - idx) % len(list)], list[(idx + current) % len(list)]
	}
}
