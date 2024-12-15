package main

import (
	"log"
	"os"
)

func main() {
	file, err := os.ReadFile("../src/inputs/day14.txt")
	if err != nil {
		log.Fatal(err)
	}
	content := string(file)
	day14_p1(content)
	day14_p2(content)

}
