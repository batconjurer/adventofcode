using System;
using System.Collections;
using System.IO;
namespace day9;

struct Position: IHashable {
    public int64 x;
    public int64 y;

    public this(int64 x, int64 y)
    {
        this.x = x;
        this.y = y;
    }

    public int GetHashCode()
    {
       return (this.x.GetHashCode() << 1) ^ this.y.GetHashCode();
    }
}


enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Instruction
{
    public Direction dir;
    public uint64 moves;

    public this(Direction dir, uint64 moves)
    {
        this.dir = dir;
        this.moves = moves;
    }

    public void Display()
    {
        switch (this.dir)
        {
            case Direction.Up:
                Console.WriteLine("Up: {}", this.moves);
            case Direction.Down:
                Console.WriteLine("Down: {}", this.moves);
            case Direction.Left:
                Console.WriteLine("Left: {}", this.moves);
            case Direction.Right:
                Console.WriteLine("Right: {}", this.moves);
            case Direction.DownRight:
                Console.WriteLine("Down-Right: {}", this.moves);
            case Direction.UpRight:
                Console.WriteLine("Up-Right: {}", this.moves);
            case Direction.DownLeft:
                Console.WriteLine("Down-Left: {}", this.moves);
            case Direction.UpLeft:
                Console.WriteLine("Up-Left: {}", this.moves);
        }
    }
}

struct RopeTrail
{
    public (int64, int64) head;
    public (int64, int64) tail;
    public HashSet<Position> visited;

    public this(int64 x, int64 y)
    {
        this.head = (x, y);
        this.tail = (x, y);
        this.visited = new HashSet<Position>();
        this.visited.Add(Position(0, 0));
    }

    public void Display()
    {
        Console.WriteLine("Head: ({}, {}), Tail: ({}, {})", this.head.0, this.head.1, this.tail.0, this.tail.1);
    }

    public List<Instruction> apply(Instruction instruction) mut
    {
        var instructions = new List<Instruction>();

        for ( uint i < instruction.moves)
        {
            /* Adjust head */
            switch (instruction.dir)
            {
                case Direction.Up:
                    this.head.0 += 1;
                case Direction.Down:
                    this.head.0 -= 1;
                case Direction.Left:
                    this.head.1 -= 1;
                case Direction.Right:
                    this.head.1 += 1;
                case Direction.UpRight:
                    this.head.0 += 1;
                    this.head.1 += 1;
                case Direction.UpLeft:
                    this.head.0 += 1;
                    this.head.1 -= 1;
                case Direction.DownRight:
                    this.head.0 -= 1;
                    this.head.1 += 1;
                case Direction.DownLeft:
                    this.head.0 -= 1;
                    this.head.1 -= 1;
            }
            /* Head above tail */
            if (this.head.0 == this.tail.0 + 2)
            {
                this.tail.0 += 1;
                /* Same column */
                if (this.head.1 == this.tail.1)
                {
                    instructions.Add(Instruction(Direction.Up, 1));

                }
                /* Head right of tail */
                else if (this.head.1 > this.tail.1)
                {
                    this.tail.1 += 1;
                    instructions.Add(Instruction(Direction.UpRight, 1));
                }
                /* Head left of tail */
                else {
                    this.tail.1 -= 1;
                    instructions.Add(Instruction(Direction.UpLeft, 1));
                }
            }
            /* Head below tail */
            else if (this.head.0 == this.tail.0 - 2)
            {
                this.tail.0 -= 1;
                /* Same column */
                if (this.head.1 == this.tail.1)
                {
                    instructions.Add(Instruction(Direction.Down, 1));
                }
                /* Head right of tail */
                else if (this.head.1 > this.tail.1)
                {
                    this.tail.1 += 1;
                    instructions.Add(Instruction(Direction.DownRight, 1));
                }
                /* Head left of tail */
                else {
                    this.tail.1 -= 1;
                    instructions.Add(Instruction(Direction.DownLeft, 1));
                }
            }
            /* Head left of tail */
            else if (this.head.1 == this.tail.1 - 2)
            {
                this.tail.1 -= 1;
                /* Same row */
                if (this.head.0 == this.tail.0)
                {
                    instructions.Add(Instruction(Direction.Left, 1));
                }
                /* Head above tail */
                else if (this.head.0 > this.tail.0)
                {
                    this.tail.0 += 1;
                    instructions.Add(Instruction(Direction.UpLeft, 1));
                }
                /* Head below tail */
                else {
                    this.tail.0 -= 1;
                    instructions.Add(Instruction(Direction.DownLeft, 1));
                }
            }
            /* Head right of tail */
            else if (this.head.1 == this.tail.1 + 2)
            {
                this.tail.1 += 1;
                /* Same row */
                if (this.head.0 == this.tail.0)
                {
                    instructions.Add(Instruction(Direction.Right, 1));
                }
                /* Head above tail */
                else if (this.head.0 > this.tail.0 )
                {
                    this.tail.0 += 1;
                    instructions.Add(Instruction(Direction.UpRight, 1));
                }
                /* Head below tail */
                else {
                    this.tail.0 -= 1;
                    instructions.Add(Instruction(Direction.DownRight, 1));
                }
            }
            this.visited.Add(Position(tail.0, tail.1));
        }
        return instructions;
    }

}

struct LongRope {
    public List<RopeTrail> rope;

    public this()
    {
        this.rope = new List<RopeTrail>();
        for (uint i < 9)
        {
            this.rope.Add(RopeTrail(0, 0));
        }
    }

    public void Display()
    {
        var segments = this.rope.GetEnumerator();
        while (segments.GetNext() case .Ok(let seg))
        {
            seg.Display();

        }
        Console.WriteLine("======");
    }

    public void apply(Instruction instruction) mut
    {
        var instructions = scope List<Instruction>();
        instructions.Add(instruction);
        var segments = this.rope.GetEnumerator();
        while (segments.GetNextRef() case .Ok(var segment))
        {
            var new_instructions = scope List<Instruction>();
            var inst_enumerator = instructions.GetEnumerator();
            while (inst_enumerator.GetNext() case .Ok(let inst))
            {
                let tail_instructions = segment.apply(inst);
                new_instructions.AddRange(tail_instructions.GetEnumerator());
            }
            instructions.Clear();
            instructions.AddRange(new_instructions.GetEnumerator());
        }
    }

    public int Visited
    {
        get
        {
            return this.rope[8].visited.Count;
        }
    }
}

class Program
{
	public static int Main(String[] args)
	{
        var file1 = scope String();
        PartOne("/home/satan/Projects/adventofcode/aoc2022/day9/test.txt", file1);
        var file2 = scope String();
        PartTwo("/home/satan/Projects/adventofcode/aoc2022/day9/input.txt", file2);
		return 0;
	}

    public static Result<void, FileError> PartOne (StringView path, String contents)
    {
        File.ReadAllText(path, contents);
        var trail = new RopeTrail(0, 0);

        var enumerator = contents.Split('\n');
        while (enumerator.GetNext() case .Ok(let line))
        {
            var line_enumerator = line.Split(' ');
            var direction = Direction.Up;
            if (line_enumerator.GetNext() case .Ok(let val))
            {
                switch (val)
                {
                    case "R":
                        direction = Direction.Right;
                    case "L":
                        direction = Direction.Left;
                    case "U":
                        direction = Direction.Up;
                    case "D":
                        direction = Direction.Down;
                    default:
                        continue;
                }
            }

            if (line_enumerator.GetNext() case .Ok(let val))
            {
                if (UInt64.Parse(val) case .Ok(let moves))
                {
                    trail.apply(Instruction(direction, moves));
                }
            }
        }
        Console.WriteLine("Number of visited squares: {}", trail.visited.Count);
        return .Ok(void());
    }

    public static Result<void, FileError> PartTwo (StringView path, String contents)
    {
        File.ReadAllText(path, contents);
        var trail = new LongRope();
        var enumerator = contents.Split('\n');

        while (enumerator.GetNext() case .Ok(let line))
        {
            var line_enumerator = line.Split(' ');
            var direction = Direction.Up;
            if (line_enumerator.GetNext() case .Ok(let val))
            {
                switch (val)
                {
                    case "R":
                        direction = Direction.Right;
                    case "L":
                        direction = Direction.Left;
                    case "U":
                        direction = Direction.Up;
                    case "D":
                        direction = Direction.Down;
                    default:
                        continue;
                }
            }
            if (line_enumerator.GetNext() case .Ok(let val))
            {
                if (UInt64.Parse(val) case .Ok(let moves))
                {
                    trail.apply(Instruction(direction, moves));
                }
            }
        }
        Console.WriteLine("Number of visited squares: {}", trail.Visited);
        return .Ok(void());
    }

}