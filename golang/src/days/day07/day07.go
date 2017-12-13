package main

import (
//	"fmt"
	"regexp"
	"strings"
	"strconv"
)

func main() {
	
}

type Program struct {
	Name string
	Weight int
	Below []string
}
//qgcmjz (87) -> skzkx, pzkofch
var Pattern = regexp.MustCompile(`([a-z]+) \((\d+)\)( -> (([a-z]+, )*[a-z]+))?`)

func parseLine(line string ) Program {
	groups := Pattern.FindStringSubmatch(line)
	weight, err := strconv.Atoi(groups[2])
	if(err != nil) {
		panic(err)
	}
	program :=  Program {
		Name : groups[1],
		Weight : weight,
	}

	if(len(groups) > 2) {
		program.Below = strings.Split(groups[3], ", ")
	}

	return program
}
