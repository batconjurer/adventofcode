package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
    "strings"
)

// A squre grid which consists of cells with are
// either empty or contain a roll of paper
type PaperGrid struct {
    inner [][]bool
}

func newPaperGrid(grid [][]bool) *PaperGrid {
    return &PaperGrid{inner: grid}
}

// Prune rolls touching < 4 other rolls. Returns the number
// of rolls pruned
func (grid *PaperGrid) prune() int {
    count := 0
    pruned := make([][]bool, len(grid.inner), len(grid.inner))
    for row := 0; row < len(grid.inner); row++ {
        pruned[row] = make([]bool, len(grid.inner[0]), len(grid.inner[0]))

        for col := 0; col < len(grid.inner[0]); col++ {
            if !grid.inner[row][col] {
                continue
            }
            if grid.countNeighbors(row, col) < 4 {
                count += 1
                pruned[row][col] = true
            }
        }
    }

    for row := 0; row < len(grid.inner); row++ {
        for col := 0; col < len(grid.inner[0]); col++ {
            if pruned[row][col] {
                grid.inner[row][col] = false
            }
        }
    }
    return count
}

// count the number of rolls adjacent to this cell
func (grid *PaperGrid) countNeighbors(row, col int) int {
    if !grid.boundsCheck(row, col) {
        log.Fatalf("Tried to access grid space that was out of bounds")
    }
    count := 0
    if grid.boundsCheck(row - 1, col) && grid.inner[row - 1][col] {
        count += 1
    }
    if grid.boundsCheck(row - 1, col - 1) && grid.inner[row - 1][col - 1] {
        count += 1
    }
    if grid.boundsCheck(row - 1, col + 1) && grid.inner[row - 1][col + 1] {
        count += 1
    }
    if grid.boundsCheck(row, col - 1) && grid.inner[row][col - 1] {
        count += 1
    }
    if grid.boundsCheck(row, col + 1) && grid.inner[row][col + 1] {
        count += 1
    }
    if grid.boundsCheck(row + 1, col) && grid.inner[row + 1][col] {
        count += 1
    }
    if grid.boundsCheck(row + 1, col - 1) && grid.inner[row + 1][col - 1] {
        count += 1
    }
    if grid.boundsCheck(row + 1, col + 1) && grid.inner[row + 1][col + 1] {
        count += 1
    }
    return count

}

func (grid *PaperGrid) boundsCheck(row, col int) bool {
    return !(row < 0 || row >= len(grid.inner) || col < 0 || col >= len(grid.inner[0]))
}


func parseInput(filename string) *PaperGrid {
    file, err := os.Open(filename)
    if err != nil {
        log.Fatalf("Failed to open file: %s", err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    grid := make([][]bool, 0, 139)
     // Iterate over each line
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        var row = make([]bool, 0, len(line))
        for _, r := range line {
            row = append(row, r == '@')
        }
        grid = append(grid, row)
    }

    // Check for errors during scanning
    if err := scanner.Err(); err != nil {
        log.Fatalf("Error reading file: %s", err)
    }
    return newPaperGrid(grid)
}

// count how many rolls can be pruned in a single pass
func partOne(filename string) {
    grid := parseInput(filename)
    count := grid.prune()
    fmt.Println("Part one: ", count)
}

// keep pruning until no longer possible. Count total of pruned rolls
func partTwo(filename string) {
    grid := parseInput(filename)
    total := 0
    for {
        count := grid.prune()
        if count == 0 {
            break
        } else {
            total += count
        }
    }
    fmt.Println("Part two: ", total)
}

func main() {
    partOne("input.txt")
    partTwo("input.txt")
}