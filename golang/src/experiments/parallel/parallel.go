package main

import (
	"days/day20"
	"os"
	"time"
	"fmt"
	"utils"
)


func main() {
	args := os.Args[1:]
	threadcount := util.MustAtoi(args[0])
	iters := util.MustAtoi(args[1])

	allParticles := day20.ParseInput("inputs/day20.txt")

	//break up slices
	sliceSize := len(allParticles) / threadcount
	channels := make([]chan day20.Particle, threadcount)
	
	start := time.Now()
	for idx := 0; idx < threadcount; idx++ {
		slice := allParticles[(idx*sliceSize):(idx*sliceSize)+sliceSize]
		channels[idx] = make(chan day20.Particle)
		go day20.RunSimulationParallel(slice, iters, channels[idx])
	}

	var minParticle day20.Particle
	for _, channel := range channels {
		part := <- channel
		if minParticle == (day20.Particle{}) {
			minParticle = part
		} else if part.ManhattanDistance() < minParticle.ManhattanDistance() {
			minParticle = part
		}
	}
	
	elapsed := time.Since(start)
	fmt.Printf("Thread count: %d, Iterations: %d, Time Elapsed: %s\n", threadcount, iters, elapsed)
	fmt.Printf("Min Particle: %+v\n", minParticle)
}
