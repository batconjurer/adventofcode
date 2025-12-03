package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
    "strings"
)

var (
    part_one int = 0
    part_two int = 0
)

func parseInput(filename string, callback func([]int) ) {
    file, err := os.Open(filename)
    if err != nil {
        log.Fatalf("Failed to open file: %s", err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

     // Iterate over each line
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        var bank = make([]int, 0, len(line))
        for _, r := range line {
            cell := int(r - '0')
            bank = append(bank, cell)
        }
        callback(bank)
    }

    // Check for errors during scanning
    if err := scanner.Err(); err != nil {
        log.Fatalf("Error reading file: %s", err)
    }
}

func partOne(bank []int) {
    part_one += findMaxNumWithNDigits(bank, 2)
}

func partTwo(bank []int) {
    part_two += findMaxNumWithNDigits(bank, 12)
}

// Given a slice of positive ints, find the largest
// n digit sub-slice without reordering
func findMaxNumWithNDigits(bank []int, n int) int {
    // base of recursion
    if n == 0 {
        return 0
    }
    max_index := 0
    max_value := 0
    // we find the largest number the first (len(bank) - (n -1)) entries
    for i:= 0; i < len(bank) - (n - 1); i++ {
        if bank[i] > max_value {
            max_index = i
            max_value = bank[i]
        }
    }

    return max_value * PowInts(10, n - 1) + findMaxNumWithNDigits(bank[max_index + 1:], n - 1)
}

// exponentiation
func PowInts(x, n int) int {
   if n == 0 { return 1 }
   if n == 1 { return x }
   y := PowInts(x, n/2)
   if n % 2 == 0 { return y*y }
   return x*y*y
}

func main() {
    parseInput("input.txt", partOne)
    fmt.Println("Part one: ", part_one)
    parseInput("input.txt", partTwo)
    fmt.Println("Part two: ", part_two)
}