

defmodule Digitize do
  defp to_int(digit) do
    case digit do
      "one" -> "o1e"
      "two" -> "t2o"
      "three" -> "t3e"
      "four" -> "f4r"
      "five" -> "f5e"
      "six" -> "s6x"
      "seven" -> "s7n"
      "eight" -> "e8t"
      "nine" -> "n9n"
    end
  end

  def digitize(line) do
    digify(line, false)
  end

  def digify(line, done) when done != true do
    d = String.replace(line, ~r/(one|two|three|four|five|six|seven|eight|nine)/, fn(digit) -> to_int(digit) end)
    digify(d, d == line)
  end

  def digify(line, done) when done do
    line
  end

end

defmodule Calibrate do
  def extract(line) do
    numbers = String.replace(String.upcase(line), ~r/[A-Z]/, "")
    String.to_integer(String.first(numbers) <> String.last(numbers))
  end

  def part_one(file) do
    IO.puts("Part one\n")
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    total = Enum.map(lines, fn(a) -> extract(a) end) |> Enum.sum()
    IO.puts(total)
    IO.puts("")
  end

  def part_two(file) do
    IO.puts("Part two\n")
    contents = File.read!(file)
    lines = String.split(contents, "\n")
    total = Enum.map(lines,fn(a) ->extract(Digitize.digitize(a))end) |> Enum.sum()
    IO.puts(total)
    IO.puts("")
  end

end

Calibrate.part_one("input.txt")
Calibrate.part_two("input.txt")

