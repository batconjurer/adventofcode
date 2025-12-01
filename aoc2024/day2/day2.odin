package main

import "core:fmt"
import "core:sort"
import "core:math"
import "core:os"
import "core:strings"
import "core:strconv"

part1 :: proc(file: string) {
    safe := 0
    data, ok := os.read_entire_file(file, context.allocator)
    if !ok {
    // could not read file
        return
    }
    defer delete(data, context.allocator)

    it := string(data)
    for line in strings.split_lines_iterator(&it) {
        levels := [dynamic]int{};
        parts, _ := strings.split(line, " ", context.allocator)
        for p in parts {
            level, _ := strconv.parse_int(p)
            append(&levels, level)
        }
        if is_report_safe(levels[:], -1) {
            safe += 1
        }
    }
    fmt.println("Part 1: ", safe)
}

part2 :: proc(file: string) {
    safe := 0
    data, ok := os.read_entire_file(file, context.allocator)
    if !ok {
    // could not read file
        return
    }
    defer delete(data, context.allocator)

    it := string(data)
    for line in strings.split_lines_iterator(&it) {
        levels := [dynamic]int{};
        parts, _ := strings.split(line, " ", context.allocator)
        for p in parts {
            level, _ := strconv.parse_int(p)
            append(&levels, level)
        }
        if is_dampened_report_safe(levels[:]) {
            safe += 1
        }
    }
    fmt.println("Part 2: ", safe)
}

// Determine if the sequence is increasing or decreasing, taking into account
// the possibiliy of skipping entries
get_positivity :: proc(levels: []int, skip: int) -> (invalid: bool, is_positive: bool) {
    invalid = false
    is_positive = true
    if skip == 0 {
        if len(levels) > 2 {
            invalid = levels[1] == levels[2]
            is_positive = levels[1] < levels[2]
        }
    } else if skip == 1 {
        if len(levels) > 2 {
            invalid = levels[0] == levels[2]
            is_positive = levels[0] < levels[2]
        }
    } else {
        invalid = levels[0] == levels[1]
        is_positive = levels[0] < levels[1]
    }
    return
}

// Check if a report is safe, skipping the entry at position `skip`
//
// A report is safe if it is strictly monotonically increasing or
// decreasing and consecutive differences are between 1 and 3.
is_report_safe :: proc(levels: []int, skip: int) -> bool {
    invalid, is_positive := get_positivity(levels, skip)
    if invalid {
        return false
    }
    for i:=0; i < len(levels) - 1; i+=1 {
        if i == skip {
            continue
        }
        first:= levels[i]
        second := 0
        if i+1 == skip {
            if i+2 == len(levels) {
                continue
            } else {
                second = levels[i+2]
            }
        } else {
            second = levels[i+1]
        }
        if first < second != is_positive {
            return false
        }
        if diff :=  math.abs(first - second); !(1 <= diff && diff <= 3) {
            return false
        }
    }
    return true
}

// Check if removing at most one level makes a report safe
is_dampened_report_safe :: proc(levels: []int) -> bool {
    for i:=0; i < len(levels); i+=1 {
        if is_report_safe(levels, i) {
            return true
        }
    }
    return false
}

main :: proc() {
    part1("input.txt")
    part2("input.txt")
}