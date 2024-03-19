defmodule DayEight do
  def parse(file) do
    contents = File.read!(file)
    [moves | lines] = String.split(contents, "\n")
    moves = Stream.cycle(String.graphemes(moves))
    {_ ,map} = Enum.map_reduce(lines, %{},fn(line, acc) -> {line, parse_line(line, acc)} end)
    {moves, map}
  end

  def parse_line(line, map) do
    [key, nodes] = String.split(line, "=")
    key = String.replace(key, " ", "")
    nodes = String.replace(nodes, "(", "")
    nodes = String.replace(nodes, ")", "")
    nodes = String.replace(nodes, " ", "")
    [left, right] = String.split(nodes, ",")
    Map.put(map, key, {left, right})
  end

  def part_one(file) do
    steps = find_steps(file, "AAA")
    IO.puts("Part one #{steps}")
  end

  def part_two(file) do
    steps1 = find_steps(file, "KLA")
    steps2 = find_steps(file, "AAA")
    steps3 = find_steps(file, "NDA")
    steps4 = find_steps(file, "LBA")
    steps5 = find_steps(file, "NNA")
    steps6 = find_steps(file, "QVA")
    ans = BasicMath.lcm(steps1, steps2)
    ans = BasicMath.lcm(ans, steps3)
    ans = BasicMath.lcm(ans, steps4)
    ans = BasicMath.lcm(ans, steps5)
    ans = BasicMath.lcm(ans, steps6)
    IO.puts("Part two: #{ans}")
  end

  def find_steps(file, start) do
    {moves, map} = parse(file)
    next = start
    traversal = Stream.transform(moves, fn -> %{next: next, map: map, steps: 0} end, fn(move, acc) ->
        node = traverse(Map.get(acc, :map), Map.get(acc, :next), move)
        map = Map.put(acc, :next, node)
        steps = Map.get(acc, :steps)
        map = Map.put(map, :steps, steps + 1)
        if List.last(String.graphemes(node)) == "Z" do
          {:halt, map}
        else
          {[move], map}
        end
      end,
      fn(acc) ->
        steps = Map.get(acc, :steps)
      end
    )
    length(Enum.to_list(traversal)) + 1
  end

  def traverse(map, start_node, move) do
    {left, right} = Map.get(map, start_node)
    if move == "L" do
      left
    else
      right
      end
  end
end

defmodule BasicMath do
  def gcd(a, 0), do: a
  def gcd(0, b), do: b
  def gcd(a, b), do: gcd(b, rem(trunc(a),b))

  def lcm(0, 0), do: 0
  def lcm(a, b), do: (a*b)/gcd(a,b)
end



DayEight.part_one("input.txt")
DayEight.part_two("input.txt")
