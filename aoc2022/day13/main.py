from enum import Enum

class Cmp(Enum):
    LESS = 1
    EQUAL = 2
    GREATER = 3

class Signal:

    def __init__(self, signal):
        self.signal = signal

    def __lt__(self, other):
        return less(self.signal, other.signal) == Cmp.LESS

    def __eq__(self, other):
        return self.signal == other.signal


def less(left, right):
    if isinstance(left, list) and isinstance(right, list):
        for (left_val, right_val) in zip(left, right):
            cmp = less(left_val, right_val)
            if cmp == Cmp.GREATER:
                return cmp
            elif cmp == Cmp.LESS:
                return cmp
        if len(left) == len(right):
            return Cmp.EQUAL
        elif len(left) < len(right):
            return Cmp.LESS
        else:
            return Cmp.GREATER
    elif isinstance(left, list) and isinstance(right, int):
        return less(left, [right])
    elif isinstance(left, int) and isinstance(right, list):
        return less([left], right)
    else:
        if left < right:
            return Cmp.LESS
        elif left == right:
            return Cmp.EQUAL
        else:
            return Cmp.GREATER

def parse_part_one(filename):
    pairs = []
    with open(filename) as file:
        lines = [line for line in file.readlines() if line != "\n"]
        for (left, right) in zip(lines[:-1:2], lines[1::2]):
            pairs.append((eval(left), eval(right)))

    return pairs


def part_one():
    pairs = parse_part_one("input.txt")
    counter = 0
    for (index, pair) in enumerate(pairs):
        if less(pair[0], pair[1]) == Cmp.LESS:
            counter += index + 1

    print("Part one: ", counter)

def part_two():
    with open("input.txt") as file:
        signals = [Signal(eval(line)) for line in file.readlines() if line != "\n"]

    signals += [Signal([[2]]), Signal([[6]])]
    signals.sort()
    ixs = [ix + 1 for (ix, sig) in enumerate(signals) if sig.signal == [[2]] or sig.signal == [[6]]]
    score = 1
    for ix in ixs:
        score *= ix
    print("Part two: ", score)


if __name__ == "__main__":
    part_one()
    part_two()

