defmodule DayTwo do
  def part_one(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    Enum.map(lines, fn(line) -> possible(line) end) |> Enum.sum()
  end

  def part_two(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    Enum.map(lines, fn(line) -> power(line) end) |> Enum.sum()
  end

  def possible(line) do
    [id | tail] = String.split(line, ":")
    [_ | [id| _] ]= String.split(id)
    id = String.to_integer(id)
    sets = String.split(List.first(tail), ";")
    if Enum.all?(sets,
      fn(set) ->
        nums = count(String.split(set, ","))
        nums[:red] <= 12 and nums[:blue] <= 14 and nums[:green] <= 13
                     end
    ) do
      id
    else
      0
    end
  end

  def power(line) do
    [_ | tail] = String.split(line, ":")
    sets = String.split(List.first(tail), ";")
    maxs = max_each(sets)
    maxs[:red] * maxs[:blue] * maxs[:green]
  end

  def max_each(sets) when sets != [] do
    [set | tail] = sets
    nums = count(String.split(set, ","))
    rest = max_each(tail)
    [red: max(nums[:red], rest[:red]), blue: max(nums[:blue], rest[:blue]), green: max(nums[:green], rest[:green])]
  end

  def max_each(sets) when sets == [] do
    [red: 0, blue: 0, green: 0]
  end

  def count(set) when set != [] do
    [head | tail] = set
    [num, color] = String.split(head, " ", [trim: true])

    num = String.to_integer(num)
    nums = count(tail)
    case color do
      "red" ->
        [red: nums[:red] + num, blue: nums[:blue], green: nums[:green]]
      "blue" ->
        [red: nums[:red], blue: nums[:blue] + num, green: nums[:green]]
      "green" ->
        [red: nums[:red], blue: nums[:blue], green: nums[:green] + num]
    end
  end

  def count(set) when set == [] do
    [red: 0, blue: 0, green: 0]
  end

end

IO.puts(DayTwo.part_one("input.txt"))
IO.puts(DayTwo.part_two("input.txt"))