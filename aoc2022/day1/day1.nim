import strutils

let input = readFile("input.txt")

var max_calories: seq[uint] = @[0'u, 0, 0]
for elf in input.split("\n\n"):
    var calories: uint = 0
    for calorie in elf.splitLines():
        calories += parseUInt(calorie)
    
    if calories > max_calories[0]:
        max_calories.insert(calories, 0)
    elif calories > max_calories[1]:
        max_calories.insert(calories, 1)
    elif calories > max_calories[2]:
        max_calories.insert(calories, 2)
    max_calories = max_calories[0..<3]


var sum = 0'u
for calorie in max_calories:
    sum += calorie

echo sum

