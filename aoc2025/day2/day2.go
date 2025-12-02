package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
    "strconv"
    "strings"
)

var (
    part_one uint64 = 0
    ranges [][2]uint64
    max_high uint64 = 0
    min_low uint64 = 10000000000
)

func parseInput(filename string, callback func(uint64, uint64) ) {

    file, err := os.Open(filename)
    if err != nil {
        log.Fatalf("Failed to open file: %s", err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

     // Iterate over each line
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        for range_str := range strings.SplitSeq(line, ",") {
            parts := strings.Split(range_str, "-")
            low, err := strconv.ParseUint(parts[0], 10, 64)
            if err != nil {
                log.Fatalf("Failed to parse input", err)
            }
            high, err := strconv.ParseUint(parts[1], 10, 64)
            if err != nil {
                log.Fatalf("Failed to parse input", err)
            }
            callback(low, high)
        }
    }

    // Check for errors during scanning
    if err := scanner.Err(); err != nil {
        log.Fatalf("Error reading file: %s", err)
    }
}

func partOne(low uint64, high uint64) {
    digits_low := countDigits(low)
    digits_high := countDigits(high)
    // a range with only numbers with odd digits count
    if digits_high == digits_low && digits_high % 2 != 0 {
        return
    }

    var min uint64 = 0
    var max uint64 = 0
    if digits_low % 2 == 0 {
        half_digits := PowInts(10, digits_low / 2)
        // split the low into both halves and take the min. The search starts from here
        min =  Min( low / half_digits, low % half_digits )
    } else {
        // we round up to the next power of ten, split into halves and start the search there
        min = PowInts(10, digits_low / 2)
    }

    if digits_high % 2 == 0 {
        half_digits := PowInts(10, digits_high / 2)
        // split the high into both halves and take the max. The search ends here
        max =  Max( high / half_digits, high % half_digits )
    } else {
        // we round down to the next power of ten, ending the search one before this number
        max = PowInts(10, digits_high / 2) - 1
    }

    // for each candidate number, concatenate it with itself and make sure it is still in range
    for j := min; j <= max; j++ {
        doubled := dup(j)
        if doubled > high {
            // all numbers afterwards will also be too high so we can stop here
            return
        }
        if doubled >= low {
            part_one += doubled
        }
    }

}

func partTwo(filename string) {
    parseInput(filename, storeRanges)
    var part_two uint64 = 0
    found := make(map[uint64]bool)
    for candidate := uint64(1); candidate <= 99999; candidate++ {
        digits := countDigits(candidate)
        // the maximum number of times the number can be concatenated
        // without exceeding ten digits (we know the max is less than 10 ^ 10)
        max_dup := 10 / digits
        for j := uint64(2); j <= max_dup; j++ {
            try := ndup(candidate, j, digits)
            // this method can create candidates multiple times. We only want
            // them to contribute to the sum once
            if found[try] {
                continue
            } else {
                found[try] = true
            }
            if filterRanges(try) {
                part_two += try
            }
        }
    }
    fmt.Println("Part two: ", part_two)
}

func storeRanges(low uint64, high uint64) {
    max_high = Max(max_high, high)
    min_low = Min(min_low, low)
    new_range := [2]uint64{low, high}
    ranges = append(ranges, new_range)
}

// check if a number is within the any of the ranges
func filterRanges(num uint64) bool {
    for i := 0; i < len(ranges); i++ {
        if ranges[i][0] <= num && num <= ranges[i][1] {
            return true
        }
    }
    return false
}

// count the number of digits in the input (base 10)
func countDigits(num uint64) uint64 {
    if num == 0 {
        return 1
    }
    var count uint64 = 0
    for num != 0 {
        num /= 10
        count++
    }
    return count
}

// exponentiation
func PowInts(x, n uint64) uint64 {
   if n == 0 { return 1 }
   if n == 1 { return x }
   y := PowInts(x, n/2)
   if n % 2 == 0 { return y*y }
   return x*y*y
}

func Max(x, y uint64) uint64 {
    if x < y {
        return y
    } else {
        return x
    }
}

func Min(x, y uint64) uint64 {
    if x > y {
        return y
    } else {
        return x
    }
}

// duplicate the digits of a number (base 10)
func dup(num uint64) uint64 {
    digits := countDigits(num)
    return PowInts(10, digits) * num + num
}


// concatenate a number with `digits` digits with itself `times` times
func ndup(num, times, digits uint64) uint64 {
    var sum uint64 = 0
    for i := uint64(0); i < times; i++ {
        sum += num * PowInts(10, i * digits)
    }
    return sum
}

func main() {
    parseInput("input.txt", partOne)
    fmt.Println("Part one: ", part_one)
    partTwo("input.txt")
}