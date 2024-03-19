defmodule Array do
  defstruct [:inner, :rows, :cols]
  @type inner :: [[String]]
  @type rows :: integer
  @type cols :: integer

  def new(inner) do
    rows = length(inner)
    cols = length(List.first(inner, []))
    %Array {
      inner: inner,
      rows: rows,
      cols: cols,
    }
  end

  def at(array, pos) do
    %Array{inner: inner, rows: rows, cols: cols} = array
    {row, col} = pos
    if row >= rows or col >= cols do
       "out"
    else
      enum_at(enum_at(inner, row), col)
    end
  end

  def enum_at([_ | t] = _, i) when i > 0  do
    enum_at(t, i-1)
  end

  def enum_at([h | _], _), do: h

end

defmodule Heading do
  defstruct [:pos, :dir]
  @type pos :: {integer, integer}
  @type direction :: :north | :south | :east | :west
end

defmodule DaySixteen do
  def parse(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    Array.new(Enum.map(lines, fn(line) -> String.graphemes(String.trim(line)) end))
  end

  def part_one(file) do
    array = parse(file)
    visited = %{}
    to_visit = [%Heading{pos: {0, -1}, dir: :east}]
    visited = step(to_visit, visited, array)
    {_, visited} = Enum.map_reduce(Map.keys(visited), %{}, fn(heading, acc) -> {heading, Map.put(acc, heading.pos, 1)} end)
    charged = length(Map.keys(visited))
    IO.puts("Part one: #{charged}")
  end

  def part_two(file) do
    array = parse(file)
    rows = Enum.to_list(0..array.rows - 1)
    cols = Enum.to_list(0..array.cols - 1)
    left_positions = Enum.map(rows, fn(row) -> %Heading{pos: {row, -1}, dir: :east} end)
    right_positions = Enum.map(rows, fn(row) -> %Heading{pos: {row, array.cols}, dir: :west} end)
    top_positions = Enum.map(cols, fn(col) -> %Heading{pos: {-1, col}, dir: :south} end)
    bottom_positions = Enum.map(cols, fn(col) -> %Heading{pos: {array.rows, col}, dir: :north} end)
    positions = left_positions ++ right_positions ++ top_positions ++ bottom_positions
    max = Enum.map(positions, fn(pos) ->
      visited = step([pos], %{}, array)
      {_, visited} = Enum.map_reduce(Map.keys(visited), %{}, fn(heading, acc) -> {heading, Map.put(acc, heading.pos, 1)} end)
      length(Map.keys(visited))
    end) |> Enum.max()
    IO.puts("Part two: #{max}")
  end

  def step(to_visit, visited, array) when to_visit != [] do
    [h | t] = to_visit
    neighbors = Enum.filter(neighbors(h, array), fn(n) -> !Map.has_key?(visited, n) end)
    {_, visited} = Enum.map_reduce(neighbors, visited, fn(n, acc) -> {n, Map.put(acc, n , 1)} end)
    step(t ++ neighbors, visited, array)
  end

  def step(to_visit, visited, _) when to_visit == [] do
    visited
  end

  def neighbors(%Heading{pos: {row, col}, dir: dir} = _, array) do
    rows = array.rows - 1
    cols = array.cols - 1
    case {row, col, dir} do
      {0, _, :north} -> []
      {_, 0, :west} -> []
      {^rows, _, :south} -> []
      {_, ^cols, :east} -> []
      {r, c, :north} ->
        case Array.at(array, {r - 1, c}) do
          "/" -> [%Heading{pos: {r - 1, c}, dir: :east}]
          "\\" -> [%Heading{pos: {r - 1, c}, dir: :west}]
          "-" -> [%Heading{pos: {r - 1, c}, dir: :west}, %Heading{pos: {r - 1, c}, dir: :east}]
          "out" -> IO.puts("Out of bounds!!!!")
          _ -> [%Heading{pos: {r - 1, c}, dir: :north}]
        end
      {r, c, :west} ->
        case Array.at(array, {r, c - 1}) do
          "/" -> [%Heading{pos: {r, c - 1}, dir: :south}]
          "\\" -> [%Heading{pos: {r, c - 1}, dir: :north}]
          "|" -> [%Heading{pos: {r, c - 1}, dir: :north}, %Heading{pos: {r, c - 1}, dir: :south}]
          "out" -> IO.puts("Out of bounds!!!!")
          _ -> [%Heading{pos: {r, c - 1}, dir: :west}]
        end
      {r, c, :south} ->
        case Array.at(array, {r + 1, c}) do
          "/" -> [%Heading{pos: {r + 1, c}, dir: :west}]
          "\\" -> [%Heading{pos: {r + 1, c}, dir: :east}]
          "-" -> [%Heading{pos: {r + 1, c}, dir: :east}, %Heading{pos: {r + 1, c}, dir: :west}]
          "out" -> IO.puts("Out of bounds!!!!")
          _ -> [%Heading{pos: {r + 1, c}, dir: :south}]
        end
      {r, c, :east} ->
        case Array.at(array, {r, c + 1}) do
          "/" -> [%Heading{pos: {r, c + 1}, dir: :north}]
          "\\" -> [%Heading{pos: {r, c + 1}, dir: :south}]
          "|" -> [%Heading{pos: {r, c + 1}, dir: :south}, %Heading{pos: {r, c + 1}, dir: :north}]
          "out" -> IO.puts("Out of bounds!!!!")
          _ -> [%Heading{pos: {r, c + 1}, dir: :east}]
        end
    end
  end
end


DaySixteen.part_one("input.txt");
DaySixteen.part_two("input.txt");