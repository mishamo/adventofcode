package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatalf("Failed to open file: %v", err)
	}

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var count = 0
	var sums []int
	for scanner.Scan() {
		line := scanner.Text()

		if strings.TrimSpace(line) == "" {
			sums = append(sums, count)
			count = 0
		} else {
			number, err := strconv.Atoi(line)
			if err != nil {
				log.Fatalf("Failed to read line: %v", err)
			}
			count += number
		}
	}

	sort.Ints(sums)
	println(sums[len(sums)-1] + sums[len(sums)-2] + sums[len(sums)-3])

	err = file.Close()
	if err != nil {
		log.Fatalf("Failed to close file: %v", err)
	}
}
