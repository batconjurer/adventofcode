
# A instruction for moving crates between stacks.
struct Instruction
  property quantity, from, to

  def initialize(@quantity : UInt8, @from : UInt8, @to : UInt8)
  end

  def quantity
    @quantity
  end

  def from
    @from
  end

  def to
    @to
  end
end

# A stack of crates
class Stack 
  def initialize(crates : String)
    @crates = crates.chars
  end

  def pop() : Char
    @crates.pop()
  end

  def push(char : Char)
    @crates << char
  end
end

class Yard
  def initialize(input : Array(String))
    @stacks = [] of Stack
    input.each do |str| 
      @stacks << Stack.new(str)
    end
  end

  def apply(instruction : Instruction)
    (1..instruction.quantity).each do |_|
      char = @stacks[instruction.from].pop()
      @stacks[instruction.to].push(char)
    end
  end

  def apply_batched(instruction : Instruction)
    temp = [] of Char
    (1..instruction.quantity).each do |_|
      char = @stacks[instruction.from].pop()
      temp << char
    end
    temp = temp.reverse()
    temp.each do |char|
      @stacks[instruction.to].push(char)
    end
  end

  def tops() : Array(Char)
    tops = [] of Char
    @stacks.each do |stack|
      tops << stack.pop()
    end
    tops
  end

end

instructions = [] of Instruction
File.each_line("input.txt") do |line|
  words = line.split(' ')
  instructions << Instruction.new(words[1].to_u8, words[3].to_u8 - 1, words[5].to_u8 - 1)
end
instructions.reverse()

p! "========== Part One =========="
yard = Yard.new(["FHBVRQDP", "LDZQWV", "HLZQGRPC", "RDHFJVB", "ZWLC", "JRPNTGVM", "JRLVMBS", "DPJ", "DCNWV"])
instructions.each do |instruction|
  yard.apply(instruction)
end
p! yard.tops()
p! "========== Part Two =========="
yard = Yard.new(["FHBVRQDP", "LDZQWV", "HLZQGRPC", "RDHFJVB", "ZWLC", "JRPNTGVM", "JRLVMBS", "DPJ", "DCNWV"])
instructions.each do |instruction|
  yard.apply_batched(instruction)
end
p! yard.tops()