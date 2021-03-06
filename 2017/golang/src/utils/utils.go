package util

import (
	"fmt"
	"bufio"
	"os"
	"path"
	"errors"
	"strings"
	"strconv"
	"math"
	"time"
)

func MustReadInput(filename string) []string {
	file, err := findFile(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if scanner.Err() != nil {
		panic(scanner.Err())
	}
	return lines
}
func ReadInput(filename string) ([]string, error) {
	file, err := findFile(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}


func findFile(filename string) (*os.File, error) {
	var file *os.File
	var err error
	file, err = os.Open(filename)
	if(err == nil) {
		return file, nil
	}
	gopaths := strings.Split(os.Getenv("GOPATH"), string(os.PathListSeparator))
	for _, gopath := range gopaths {
		file, err = os.Open(path.Join(gopath, filename))
		if(err == nil) {
			return file, nil
		}
	}
	return nil, errors.New("Could not find file on gopath")
}

func StringsToInts(numbers []string) []int {
	var err error
	ints := make([]int, len(numbers), cap(numbers))
	for idx, number := range numbers {
		ints[idx], err = strconv.Atoi(number)
		if(err != nil) {
			fmt.Println(err)
			panic(err)
		}
	}

	return ints
}

func MaxValueAndIdx(slice []int) (int, int) {
	var max int = math.MinInt32
	var maxIdx int = -1;
	for idx, value := range slice {
		if value > max {
			max = value
			maxIdx = idx
		}
	}
	return max, maxIdx
}

func MinValueAndIdx(slice []int) (int, int) {
	var min int = math.MinInt32
	var minIdx int = -1;
	for idx, value := range slice {
		if value < min {
			min = value
			minIdx = idx
		}
	}
	return min, minIdx
}

func Abs(a int) int {
	if a < 0 {
		return -a
	} else {
		return a
	}
}


func Sum(list *[]int) int {
	var sum int
	for _, el := range *list {
		sum += el
	}
	return sum
}

func MustAtoi(number string) int {
	num, err := strconv.Atoi(strings.TrimSpace(number));
	if(err != nil) {
		panic(err)
	}
	return num
}

func TimeTrack(start time.Time, message string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", message, elapsed)
}
