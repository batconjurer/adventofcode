defmodule DayFour do
  def part_one(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    Enum.map(lines, fn(line) ->
      score = score(line)
      if score == 0 do 0 else :math.pow(2, score - 1) end
      end) |> Enum.sum()
  end

  def part_two(file) do
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    {_, won} = Enum.map_reduce(lines, %{1 => 1}, fn(line, acc) -> {line, won_cards(line, acc)} end)
    Enum.sum(Map.values(won)) - 1
  end

  def score(card) do
    [ _ , card ] = String.split(card, ":")
    [ winners , nums ] = String.split(card, "|")
    winners = String.split(winners, " ", [trim: true])
    nums = String.split(nums, " ", [trim: true])
    intersection(winners, nums)
  end

  def won_cards(card, cards) do
    [id, _] = String.split(card, ":")
    [_, id] = String.split(id)
    id = String.to_integer(id)
    score = score(card)
    won = if score == 0  do
      {_, won} = Map.get_and_update(cards, id+1, fn(val) ->{val, if val == nil do 1 else val end} end)
      won
    else
      int = score + id
      number = Map.get(cards, id, 1)
      {k, won} = Enum.map_reduce(
        id+1..int,
        cards,
        fn(x, acc) -> Map.get_and_update(
                        acc,
                        x,
                        fn(val) ->{val, if val == nil do number + 1 else val + number end} end
                      )
        end)
      won
    end
    won
  end

  def contains(list, elem) do
    Enum.any?(list, fn(e) -> e == elem end)
  end

  def intersection(list1, list2) when list2 != [] do
    [ first | rest ] = list2
    if contains(list1, first) do
      1 + intersection(list1, rest)
    else
      intersection(list1, rest)
    end
  end

  def intersection(_, list2) when list2 == [] do
    0
  end

end
IO.puts("Part one")
IO.puts(DayFour.part_one("input.txt"))
IO.puts("Part two")
IO.puts(DayFour.part_two("input.txt"))