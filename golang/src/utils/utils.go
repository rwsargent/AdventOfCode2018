package util

import (
	"fmt"
	"bufio"
	"os"
	"path"
	"errors"
	"strings"
	"strconv"
)

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
	gopaths := strings.Split(os.Getenv("GOPATH"), fmt.Sprintf("%b", os.PathListSeparator))
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
