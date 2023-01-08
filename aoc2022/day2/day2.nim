import strutils
import unittest

type ParseException = object of Exception

type YourMove{.pure.} = enum Rock, Paper, Scissors

type OpponentMove{.pure.} = enum Rock, Paper, Scissors

type Recommendation = tuple
    yourMove: YourMove
    theirMove: OpponentMove


proc parseYourMove(str: string): YourMove {.raises: [ParseException].} =
    case str:
        of "X":
            YourMove.Rock
        of "Y":
            YourMove.Paper
        of "Z":
            YourMove.Scissors
        else:
            raise ParseException.newException("Not a valid move, expect X, Y, or Z")

proc parseTheirMove(str: string): OpponentMove {.raises: [ParseException].} =
    case str:
        of "A":
            OpponentMove.Rock
        of "B":
            OpponentMove.Paper
        of "C":
            OpponentMove.Scissors
        else:
            raise ParseException.newException("Not a valid move, expect A, B, or C")

proc fromString(str: string): Recommendation {.raises: [ParseException].}=
    let moves = str.splitWhitespace()
    (parseYourMove(moves[1]), parseTheirMove(moves[0]))


proc score(move: Recommendation): uint = 
    if move == (YourMove.Rock, OpponentMove.Rock):
        4'u
    elif move == (YourMove.Rock, OpponentMove.Paper):
        1'u
    elif move == (YourMove.Rock, OpponentMove.Scissors):
        7'u 
    elif move == (YourMove.Paper, OpponentMove.Rock):
            8'u
    elif move == (YourMove.Paper, OpponentMove.Paper):
        5'u
    elif move == (YourMove.Paper, OpponentMove.Scissors):
        2'u
    elif move == (YourMove.Scissors, OpponentMove.Rock):
        3'u
    elif move == (YourMove.Scissors, OpponentMove.Paper):
        9'u
    else:
        6'u

proc lose(their_move: OpponentMove): YourMove =
    if their_move == OpponentMove.Rock:
        YourMove.Scissors
    elif their_move == OpponentMove.Paper:
        YourMove.Rock
    else:
        YourMove.Paper

proc tie(their_move: OpponentMove): YourMove =
    if their_move == OpponentMove.Rock:
        YourMove.Rock
    elif their_move == OpponentMove.Paper:
        YourMove.Paper
    else:
        YourMove.Scissors

proc win(their_move: OpponentMove): YourMove =
    if their_move == OpponentMove.Rock:
        YourMove.Paper
    elif their_move == OpponentMove.Paper:
        YourMove.Scissors
    else:
        YourMove.Rock



proc parseYourMovePartTwo(str: string, their_move: OpponentMove): YourMove {.raises: [ParseException].} = 
    case str:
        of "X":
            lose(their_move)
        of "Y":
            tie(their_move)
        of "Z":
            win(their_move)
        else:
            raise ParseException.newException("Not a valid move, expect X, Y, or Z")


proc scorePartTwo(str: string): uint =
    let moves = str.splitWhitespace()
    let their_move = parseTheirMove(moves[0])
    score((parseYourMovePartTwo(moves[1], their_move), their_move))



let input = readFile("input.txt")
var total_score_part_one = 0'u
var total_score_part_two = 0'u 
for rec in input.split("\n"):
    total_score_part_one += score(fromString(rec))
    total_score_part_two += scorePartTwo(rec)

echo "Score part one: ", total_score_part_one
echo "Score part two: ", total_score_part_two


suite "test parsing moves":

    test "parseYourMove":
        assert(parseYourMove("X") == YourMove.Rock)
        assert(parseYourMove("Y") == YourMove.Paper)
        assert(parseYourMove("Z") == YourMove.Scissors)
        try:
            var should_fail = parseYourMove("T")
            assert(false)
        except ParseException:
            assert(true)

    test "parseTheirMove":
        assert(parseTheirMove("A") == OpponentMove.Rock)
        assert(parseTheirMove("B") == OpponentMove.Paper)
        assert(parseTheirMove("C") == OpponentMove.Scissors)
        try:
            var should_fail = parseTheirMove("T")
            assert(false)
        except ParseException:
            assert(true)

    test "parseRecommendation":
        assert(fromString("A X")==(YourMove.Rock, OpponentMove.Rock))

    test "testScore":
        assert(score(fromString("C X")) == 7'u)

    test "testLose":
        assert(lose(OpponentMove.Rock) == YourMove.Scissors)

    test "testTie":
        assert(tie(OpponentMove.Rock) == YourMove.Rock)

    test "testWin":
        assert(win(OpponentMove.Rock) == YourMove.Paper)

    test "ParsePartTwo":
        assert(parseYourMovePartTwo("X", OpponentMove.Paper) == YourMove.Rock)
        assert(parseYourMovePartTwo("Y", OpponentMove.Paper) == YourMove.Paper)
        assert(parseYourMovePartTwo("Z", OpponentMove.Paper) == YourMove.Scissors)

    test "ScorePartTwo":
        assert(scorePartTwo("B X") == 1)
        assert(scorePartTwo("B Y") == 5)
        assert(scorePartTwo("B Z") == 9)