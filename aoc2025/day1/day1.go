package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
    "strconv"
    "strings"
)


func partOne(filename string) {

    file, err := os.Open(filename)
    if err != nil {
        log.Fatalf("Failed to open file: %s", err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

    var pos uint64 = 50
    var zeros uint64 = 0
     // Iterate over each line
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        switch line[0] {
            case 'L':
                i, err := strconv.ParseInt(line[1:], 10, 64)
                if err != nil {
                    log.Fatalf("Error reading file: %s", err)
                }
                amount := uint64(i) % 100
                pos = ( pos + ( 100 - amount ) ) % 100

            case 'R':
                i, err := strconv.ParseInt(line[1:], 10, 64)
                if err != nil {
                    log.Fatalf("Error reading file: %s", err)
                }
                amount := uint64(i) % 100
                pos = ( pos + amount ) % 100
            default:
        }

        if pos == 0 {
            zeros += 1
        }
    }

    // Check for errors during scanning
    if err := scanner.Err(); err != nil {
        log.Fatalf("Error reading file: %s", err)
    }
    fmt.Println("Part One", zeros)
}

func partTwo(filename string) {

    file, err := os.Open(filename)
    if err != nil {
        log.Fatalf("Failed to open file: %s", err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

    var pos uint64 = 50
    var password uint64 = 0
     // Iterate over each line
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        switch line[0] {
            case 'L':
                i, err := strconv.ParseInt(line[1:], 10, 64)
                if err != nil {
                    log.Fatalf("Error reading file: %s", err)
                }
                password += uint64(i) / 100
                amount := uint64(i) % 100
                new_pos := ( pos + ( 100 - amount ) ) % 100
                if pos == 0 {
                    pos = new_pos
                    continue
                }
                if new_pos > pos || new_pos == 0 {
                    password += 1
                }
                pos = new_pos

            case 'R':
                i, err := strconv.ParseInt(line[1:], 10, 64)
                if err != nil {
                    log.Fatalf("Error reading file: %s", err)
                }
                password +=  uint64(i) / 100
                amount := uint64(i) % 100
                new_pos := ( pos + amount ) % 100
                if new_pos < pos {
                    password += 1
                }
                pos = new_pos
            default:
        }

    }

    // Check for errors during scanning
    if err := scanner.Err(); err != nil {
        log.Fatalf("Error reading file: %s", err)
    }
    fmt.Println("Part two", password)
}

func main() {
    partOne("input.txt")
    partTwo("input.txt")
}