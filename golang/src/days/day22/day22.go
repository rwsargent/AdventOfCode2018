package day22

import (
	"utils"
)

type Coordinate struct {
	Row, Col int
}

type Cursor struct {
	Coord Coordinate
	DirectionIdx int
}
type State int



const	STATE_COUNT = 4
const (
	Clean State = iota 
	Weakened 
	Infected
	Flagged
)


func (this *Cursor)TurnRight() {
	this.DirectionIdx = (this.DirectionIdx +1) % len(directions)
}

func (this *Cursor)TurnLeft() {
	this.DirectionIdx = (this.DirectionIdx -1)
	if(this.DirectionIdx < 0) { // roll back to the top
		this.DirectionIdx = len(directions) -1 
	}
}

func (this *Cursor)Move() {
	directions[this.DirectionIdx](this)
}


func (this *Cursor)EvolvedInfect(nodes map[Coordinate]State) State {
	state, _ := nodes[this.Coord]
	nodes[this.Coord] = (state+1) % STATE_COUNT
	return state //return previous state, makes logic nicer
}

func (this *Cursor)Infect(nodes map[Coordinate]bool) bool {
	infected, _ := nodes[this.Coord]
	nodes[this.Coord] = !infected
	return nodes[this.Coord]
}

func EvolvedSimulation(nodes map[Coordinate]State, virus Cursor, iter int) int {
	infectedCount := 0
	for ; iter > 0; iter-- {
		previousState := virus.EvolvedInfect(nodes)
		switch previousState {
		case Clean:
			virus.TurnLeft()
		case Weakened:
			infectedCount++
		case Infected:
			virus.TurnRight()
		case Flagged:
			virus.TurnRight()
			virus.TurnRight()
		}
		virus.Move()
	}

	return infectedCount
}

func Simulation(nodes map[Coordinate]bool, virus Cursor, iter int) int {
	infectectCount := 0
	for ; iter > 0; iter-- {
		if(virus.Infect(nodes)) {
			virus.TurnLeft()
			infectectCount++
		} else {
			virus.TurnRight()
		}
		virus.Move()
	}

	return infectectCount
}

func InitializeBoard(filepath string) (map[Coordinate]bool, Cursor) {
	nodes := make(map[Coordinate]bool)
	var row, col int
	var line string
	var letter rune
	for row, line = range util.MustReadInput(filepath) {
		for col, letter = range line {
			if letter == '#' {
				nodes[Coordinate{row, col}] = true
			}
		}
	}
	
	return nodes, Cursor{Coordinate{row/2, col/2}, 0}
}

func InitializeEvolvedBoard(filepath string) (map[Coordinate]State, Cursor) {
	nodes := make(map[Coordinate]State)
	var row, col int
	var line string
	var letter rune
	for row, line = range util.MustReadInput(filepath) {
		for col, letter = range line {
			if letter == '#' {
				nodes[Coordinate{row, col}] = Infected
			}
		}
	}
	
	return nodes, Cursor{Coordinate{row/2, col/2}, 0}	
}

var directions = [...]func(*Cursor){
	func(cursor *Cursor) { //NORTH
		cursor.Coord.Row--
	},
	func(cursor *Cursor) { //EAST
		cursor.Coord.Col++
	},
	func(cursor *Cursor) { //SOUTH
		cursor.Coord.Row++
	},
	func(cursor *Cursor) { //WEST
		cursor.Coord.Col--
	},
}
func (s State)String() string {
	switch s {
	case Clean: return "C"
	case Weakened: return "W"
	case Infected: return "I"
	case Flagged : return "F"
	default: return ""
	}
}
