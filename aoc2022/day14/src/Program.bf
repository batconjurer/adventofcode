using System;
using System.Collections;
using System.IO;

namespace day14;

struct Pos : IHashable
{
    public uint32 x;
    public uint32 y;

    public this(uint32 x, uint32 y)
    {
        this.x = x;
        this.y = y;
    }

    public int GetHashCode()
    {
        return (this.x.GetHashCode() << 1) ^ this.y.GetHashCode();
    }
}

struct Cave
{
    public HashSet<Pos> Rocks;
    public HashSet<Pos> Sand;
    public uint32 MaxX;
    public uint32 MaxY;
    public uint32 MinX;
    public uint32 MinY;

    public this()
    {
        this.Rocks = new HashSet<Pos>();
        this.Sand = new HashSet<Pos>();
        this.MaxX = 0;
        this.MinX = 0;
        this.MaxY = 0;
        this.MinY =0;
    }

    public void AddRocks(List<List<(uint32, uint32)>> Corners) mut
    {
        this.Rocks = new HashSet<Pos>();
        var lines = Corners.GetEnumerator();
        while (lines.GetNext() case .Ok(let line))
        {
            var corners = line.GetEnumerator();
            var prev = ((uint32) 0, (uint32) 0);
            prev = corners.GetNext();

            while (corners.GetNext() case .Ok(let pos))
            {
                this.Draw(prev, pos);
                prev = pos;
            }
        }
    }

    public void Draw((uint32, uint32) start, (uint32, uint32) end) mut
    {
        if (start.0 == end.0) {
            var begin = (uint32) 0;
            var finish = (uint32) 0;

            if (start.1 > end.1)
            {
                begin = end.1;
                finish = start.1;
            }
            else
            {
                begin = start.1;
                finish = end.1;
            }

            while (finish >= begin)
            {
                this.Rocks.Add(Pos(start.0, begin));
                begin += 1;
            }

        }
        else if (start.1 == end.1)
        {
            var begin = (uint32) 0;
            var finish = (uint32) 0;
            if (start.0 > end.0)
            {
                begin = end.0;
                finish = start.0;
            }
            else
            {
                begin = start.0;
                finish = end.0;
            }
            while (finish >= begin)
            {
                this.Rocks.Add(Pos(begin, start.1));
                begin += 1;
            }
        }
    }

    public bool AddSand() mut {
        var pos = ((uint32) 500, (uint32) 0);
        while (true)
        {
            if (this.inVoid(Pos(pos.0, pos.1)))
                return false;
            else if (!this.Occupied(Pos(pos.0, pos.1 + 1)))
                pos = (pos.0, pos.1 + 1);
            else if (!this.Occupied(Pos(pos.0 - 1, pos.1 + 1)))
                pos = (pos.0 - 1, pos.1 + 1);
            else if (!this.Occupied(Pos(pos.0 + 1, pos.1 + 1)))
                pos = (pos.0 + 1 , pos.1 + 1);
            else
                break;
        }
        this.Sand.Add(Pos(pos.0, pos.1));
        return true;
    }

    public bool findSafety()
    {
        var pos = ((uint32) 500, (uint32) 0);
        while (true)
        {
            if (!this.Blocked(Pos(pos.0, pos.1 + 1)))
                pos = (pos.0, pos.1 + 1);
            else if (!this.Blocked(Pos(pos.0 - 1, pos.1 + 1)))
                pos = (pos.0 - 1, pos.1 + 1);
            else if (!this.Blocked(Pos(pos.0 + 1, pos.1 + 1)))
                pos = (pos.0 + 1 , pos.1 + 1);
            else
                break;
        }
        if (pos.0 == 500 && pos.1 == 0)
            return false;
        this.Sand.Add(Pos(pos.0, pos.1));
        return true;
    }

    bool Occupied(Pos pos)
    {
        if (this.Rocks.Contains(pos) || this.Sand.Contains(pos))
            return true;
        else
            return false;
    }

    bool Blocked(Pos pos)
    {
        if (this.Rocks.Contains(pos) || this.Sand.Contains(pos))
            return true;
        else if (pos.y == this.MaxY + 2)
            return true;
        else
            return false;
    }

    bool inVoid(Pos pos)
    {
        if (pos.x < this.MinX || pos.x > this.MaxX)
            return true;
        else
            return false;
    }
}

class Program
{
	public static int Main(String[] args)
	{
        partOne("/home/satan/Projects/aoc2022/day14/input.txt");
        partTwo("/home/satan/Projects/aoc2022/day14/input.txt");
		return 0;
	}

    public static void partOne(StringView filename)
    {
        var cave = parseInput(filename);
        while (cave.AddSand()) {
            continue;
        }
        Console.WriteLine("Sand: {}", cave.Sand.Count);
    }

    public static void partTwo(StringView filename)
    {
        var cave = parseInput(filename);
        while (cave.findSafety()) {
            continue;
        }
        Console.WriteLine("Sand: {}", cave.Sand.Count + 1);
    }

    public static Cave parseInput(StringView filename)
    {
        var contents = scope String();
        File.ReadAllText(filename, contents);
        var lines = contents.Split('\n');
        var cave = new Cave();
        var rocks = new List<List<(uint32, uint32)>>();

        var max_x = (uint32) 0;
        var max_y = (uint32) 0;
        var min_x = (uint32) 1000;

        while (lines.GetNext() case .Ok(let line))
        {
            var rock = new List<(uint32, uint32)>();
            var coordinates = line.Split(" -> ");
            while (coordinates.GetNext() case .Ok(let pos))
            {
                var coords = pos.Split(',');
                var x = (uint32) 0;
                var y = (uint32) 0;
                if (coords.GetNext() case .Ok(let coord))
                {
                    x = uint32.Parse(coord);
                    if (x > max_x)
                    {
                        max_x = x;
                    } else if (x < min_x) {
                        min_x = x;
                    }
                }
                if (coords.GetNext() case .Ok(let coord))
                {
                    y = uint32.Parse(coord);
                    if (y > max_y)
                    {
                        max_y = y;
                    }
                }
                rock.Add((x, y));
            }
            rocks.Add(rock);
        }

        cave.MaxX = max_x;
        cave.MaxY = max_y;
        cave.MinX = min_x;
        cave.MinY = (uint32) 0;
        cave.AddRocks(rocks);
        return *cave;
    }
}