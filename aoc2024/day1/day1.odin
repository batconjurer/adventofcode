package main

import "core:fmt"
import "core:sort"
import "core:math"
import "core:os"
import "core:strings"
import "core:strconv"

diffs :: proc(a1: []int, a2: []int) -> (sum: int) {
    sum = 0
    for i:=0; i<len(a1); i+=1 {
        sum += math.abs(a1[i] - a2[i])
    }
    return
}

parse_input :: proc(file: string) -> (array1: [dynamic]int, array2: [dynamic]int) {
    data, ok := os.read_entire_file(file, context.allocator)
    if !ok {
    // could not read file
        return
    }
    defer delete(data, context.allocator)

    it := string(data)
    for line in strings.split_lines_iterator(&it) {
        // process line
        parts, _ := strings.split(line, "   ", context.allocator)
        n1, _ := strconv.parse_int(parts[0])
        append(&array1, n1)
        n2, _ := strconv.parse_int(parts[1])
        append(&array2, n2)
    }
    return
}

similarity_score :: proc(input: int, array: []int) -> (sim: int) {
    sim = 0
    for a in array {
        if a == input {
            sim += 1
        }
    }
    sim = sim * input
    return
}

part1 :: proc(filename: string) {
    array1, array2 := parse_input(filename)
    sort.quick_sort_proc(array1[:], proc(x: int, y: int) -> int { return x - y })
    sort.quick_sort_proc(array2[:], proc(x: int, y: int) -> int { return x - y })
    ans := diffs(array1[:], array2[:])
    fmt.println(ans)
}

part2 :: proc(filename: string) {
    array1, array2 := parse_input(filename)
    sum := 0
    for input in array1 {
        sum += similarity_score(input, array2[:])
    }
    fmt.println(sum)
}

main :: proc() {
    part1("input.txt")
    part2("input.txt")
}