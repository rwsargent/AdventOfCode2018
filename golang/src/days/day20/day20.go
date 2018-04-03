package day20

import (
	"utils"
	"regexp"
)

type Vector struct {
	X, Y, Z int
}

var Pattern = regexp.MustCompile(`p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>`)

func (this *Vector) Add(vector Vector) {
	this.X += vector.X
	this.Y += vector.Y
	this.Z += vector.Z
}

type Particle struct {
	Id int
	Position, Velocity, Acceleration Vector
}

func (part *Particle) UpdateVelocity() {
	part.Velocity.Add(part.Acceleration);
}

func (part *Particle) UpdatePosition() {
	part.Position.Add(part.Velocity);
}

func (part *Particle) ManhattanDistance() int {
	return (util.Abs(part.Position.X) + util.Abs(part.Position.Y) + util.Abs(part.Position.Z))
}

func RemoveSimulationNaieve(particles []*Particle, iterations int) int {
	for iter := 0 ; iter < iterations; iter++ {
		for _, particle := range particles {
			if (particle != nil) {
				particle.UpdateVelocity()
				particle.UpdatePosition()
			}
		}

		RemoveParticlesNaieve(particles)
	}

	// count non-nil particles
	var count = 0
	for _, particle := range particles {
		if particle != nil {
			count++
		}
	}
	
	return count	
}

func RemoveParticlesNaieve(particles []*Particle) {
	for fidx, first := range particles {
		var dup bool
		for sidx, second := range particles[fidx+1:] {
			if first == nil || second == nil  {
				continue
			}
			if first.Position == second.Position {
				particles[sidx] = nil
				dup = true
			}
		}
		if dup {
			particles[fidx] = nil
		}
	}
}


func RemoveSimulation(particles []*Particle, iterations int) int {
	for iter := 0 ; iter < iterations; iter++ {
		for _, particle := range particles {
			if (particle != nil) {
				particle.UpdateVelocity()
				particle.UpdatePosition()
			}
		}

		RemoveParticles(particles)
	}

	// count non-nil particles
	var count = 0
	for _, particle := range particles {
		if particle != nil {
			count++
		}
	}
	
	return count
}

func RemoveParticles(particles []*Particle) {
	seenParticles := make(map[Vector]*Particle)

	for _, particle := range particles {
		if(particle == nil) {
			continue
		}
		if pastParticle, ok := seenParticles[particle.Position]; ok {
			particles[pastParticle.Id] = nil
			particles[particle.Id] = nil
		} else {
			seenParticles[particle.Position] = particle
		}
	}
}

func RunSimulation(particles []*Particle, iterations int) Particle {
	var minParticle Particle
	for iter := 0 ; iter < iterations; iter++ {
		if len(particles) > 0 {
			minParticle = *particles[0]
		}
		for _, particle := range particles {
			particle.UpdateVelocity()
			particle.UpdatePosition()

			if (particle.ManhattanDistance() < minParticle.ManhattanDistance()) {
				minParticle = *particle
			}
		}
	}
	return minParticle;
}

func RunSimulationParallel(particles []*Particle, iterations int, channel chan Particle) {
	min := RunSimulation(particles, iterations)
	channel <- min
}

func ParseInput(path string) []*Particle {
	lines := util.MustReadInput(path)
	particles := make([]*Particle, 0, len(lines))
	for id, line := range util.MustReadInput(path) {
		groups := Pattern.FindStringSubmatch(line)
		position := Vector{getInt(groups[1]),getInt(groups[2]),getInt(groups[3])}
		velocity := Vector{getInt(groups[4]),getInt(groups[5]),getInt(groups[6])}
		acceleration := Vector{getInt(groups[7]),getInt(groups[8]),getInt(groups[9])}

		particles = append(particles, &Particle{id, position, velocity, acceleration})
	}

	return particles
}

func getInt(num string) int {
	return util.MustAtoi(num)
}
