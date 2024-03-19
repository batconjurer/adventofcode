defmodule DaySix do
  def part_one(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    [ times,  distance ] = lines
    [_ , times ] = String.split(times, ":")
    times = Enum.map(String.split(times), fn(a) -> String.to_integer(a) end)
    [_, distance] = String.split(distance, ":")
    dists = Enum.map(String.split(distance), fn(a) -> String.to_integer(a) end)
    pairs = Enum.zip(times, dists)
    result = Enum.map(pairs, fn({a,b}) -> ways_to_win(a, b) end) |> Enum.product()
    IO.puts("Part one: #{result}")
  end

  def part_two(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    [ times,  distance ] = lines
    times = String.replace(times, " ", "")
    distance = String.replace(distance, " ", "")
    [_ , times ] = String.split(times, ":")
    times = Enum.map(String.split(times), fn(a) -> String.to_integer(a) end)
    [_, distance] = String.split(distance, ":")
    dists = Enum.map(String.split(distance), fn(a) -> String.to_integer(a) end)
    pairs = Enum.zip(times, dists)
    result = Enum.map(pairs, fn({a,b}) -> ways_to_win(a, b) end) |> Enum.product()
    IO.puts("Part two: #{result}")
  end

  def ways_to_win(time, dist) do
    disc = :math.sqrt(:math.pow(time, 2) - 4 * dist)
    val = abs(trunc((time + :math.sqrt(:math.pow(time, 2) - 4 * dist)) / 2) -
      trunc((time - :math.sqrt(:math.pow(time, 2) - 4 * dist)) / 2))
    if disc == trunc(disc) do
      val - 1
    else
      val
    end
  end
end

DaySix.part_one("input.txt")
DaySix.part_two("input.txt")