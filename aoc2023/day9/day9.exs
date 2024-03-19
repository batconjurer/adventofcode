defmodule DayNine do
  def part_one(filename) do
    contents = File.read!(filename)
    lines = String.split(contents, "\n")
    lines = Enum.map(lines, fn(a) -> parse_line(a) end)
    ans = Enum.map(lines, fn(a) -> List.last(diffs(a)) end) |> Enum.sum()
    IO.puts("Part one: #{ans}")
  end

  def part_two(filename) do
    contents = File.read!(filename)
    lines = String.split(contents, "\n")
    lines = Enum.map(lines, fn(a) -> parse_line(a) end)
    ans = Enum.map(lines, fn(a) -> List.first(backwards_diffs(a)) end) |> Enum.sum()
    IO.puts("Part two: #{ans}")
  end

  def parse_line(line) do
    Enum.map(String.split(line, " "), fn(a) -> String.to_integer(a) end)
  end

  def diffs(list) do
    pairs = List.zip([list, List.delete_at(list, 0)])
    diffs = Enum.map(pairs, fn({a, b}) -> b - a end)
    if Enum.any?(diffs, fn(a) -> a != 0 end ) do
      lower = diffs(diffs)
      List.insert_at(list, length(list), List.last(list) + List.last(lower))
    else
      List.insert_at(list, length(list), List.last(list))
    end
  end

  def backwards_diffs(list) do
    pairs = List.zip([list, List.delete_at(list, 0)])
    diffs = Enum.map(pairs, fn({a, b}) -> b - a end)
    if Enum.any?(diffs, fn(a) -> a != 0 end ) do
      lower = backwards_diffs(diffs)
      List.insert_at(list, 0, List.first(list) - List.first(lower))
    else
      List.insert_at(list, 0, List.first(list))
    end
  end
end

DayNine.part_one("input.txt")
DayNine.part_two("input.txt")