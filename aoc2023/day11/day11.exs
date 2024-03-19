defmodule DayEleven do
  def part_one(file) do
    ans = process(file, 2)
    IO.puts("Part one: #{ans}")
  end

  def part_two(file) do
    ans = process(file, 1000000)
    IO.puts("Part two: #{ans}")
  end


  def process(file, ex) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    expanded = expand_columns(file)

    {_ , map} = Enum.map_reduce(lines, %{row: 0, positions: []}, fn(line, acc) ->

      if Enum.all?(String.graphemes(line), fn(a) -> a == "." end ) do
        {line, Map.update!(acc, :row, &(&1 + ex))}
      else
        row = Map.get(acc, :row)
        positions = get_galaxies(line, row, acc)
        acc = Map.update!(acc, :row, &(&1 + 1))
        {line, Map.update!(acc, :positions, fn(_) -> positions end)}
      end
      end)

    positions = Map.get(map, :positions)
    ans = distance_pair_sum(positions, expanded, ex)

    ans
  end

  def expand_columns(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    grid = Enum.map(lines, fn(line) -> String.graphemes(line) end)
    rows = length(grid)
    cols = length(List.first(grid))
    expanded = Enum.map(0..cols - 1, fn(col) ->
      Enum.all?(0..rows - 1, fn(row) -> Enum.at(Enum.at(grid, row), col) == "." end)
    end)
    expanded
  end

  def get_galaxies(line, row, acc) do
     {_ , positions} = Enum.map_reduce(Enum.with_index(String.graphemes(line)),
       Map.get(acc, :positions),
       fn(char, acc) ->
         {char, col} = char
         if char == "#" do
           {char, List.insert_at(acc, 0, {row, col})}
         else
           {char, acc}
         end
     end)
     positions
  end

  def distance_pair_sum(positions, expanded, ex) do
    ex = ex - 1
    doubled = Enum.map(positions, fn(pos) ->
      {row, col} = pos
      Enum.map(positions, fn(pos2) ->
        {row2, col2} = pos2
        {_, to_add} = Enum.map_reduce(Enum.with_index(expanded), 0, fn(c, acc) ->
          {v, ix} = c
          if v do
            if (col < ix and ix < col2) or (col2 < ix and ix < col) do
              {c, acc + ex}
            else
              {c, acc}
            end
          else
            {c, acc}
          end
        end)
        abs(row - row2) + abs(col - col2) + to_add
      end) |> Enum.sum()
    end) |> Enum.sum()
    doubled / 2
  end
end

DayEleven.part_one("input.txt")
DayEleven.part_two("input.txt")