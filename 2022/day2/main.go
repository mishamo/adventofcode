package main

import (
	"bufio"
	"log"
	"os"
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
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, " ")
		opponent := split[0]
		me := split[1]

		if opponent == "A" { //rock 1
			if me == "X" {
				count += 3
			} else if me == "Y" {
				count += 1
			} else if me == "Z" {
				count += 2
			}
		} else if opponent == "B" { //paper 2
			if me == "X" {
				count += 1
			} else if me == "Y" {
				count += 2
			} else if me == "Z" {
				count += 3
			}
		} else if opponent == "C" { //scissors 3
			if me == "X" {
				count += 2
			} else if me == "Y" {
				count += 3
			} else if me == "Z" {
				count += 1
			}
		}

		if me == "X" {
			count += 0
		} else if me == "Y" {
			count += 3
		} else if me == "Z" {
			count += 6
		}
	}

	println(count)
}
