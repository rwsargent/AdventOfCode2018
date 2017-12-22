package main

import "fmt"

func main() {
	fmt.Println("Count: ", judge(516, 190))
	fmt.Println("Picky:", judgePicky(516, 190))
}

type Generator struct {
	Next int
	Multiplier int
}

func (generator *Generator) next() int {
	generator.Next = int((int64(generator.Next) * int64(generator.Multiplier)) % int64(2147483647))
	return generator.Next
}

func (generator *Generator) nextPicky(divisible int) int {
	for {
		n := generator.next()
		if (n % divisible) == 0 {
			return n
		}
	}
	return -1 // should never happen TM
}

func judgePicky(astart, bstart int) int {
	const mask = 0xFFFF;
	genA := Generator{astart, 16807}
	genB := Generator{bstart, 48271}
	var counter int
	for idx := 0; idx < 5000000; idx++ {
		Anum := genA.nextPicky(4)
		Bnum := genB.nextPicky(8)

		if (Anum & mask) == (Bnum & mask) {
			counter++
		}
	}
	return counter
}


func judge(astart, bstart int) int {
	const mask = 0xFFFF;
	genA := Generator{astart, 16807}
	genB := Generator{bstart, 48271}
	var counter int
	for idx := 0; idx < 40000000; idx++ {
		Anum := genA.next()
		Bnum := genB.next()

		if (Anum & mask) == (Bnum & mask) {
			counter++
		}
	}

	return counter
}
