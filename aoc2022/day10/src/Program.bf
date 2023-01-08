using System;
using System.Collections;
using System.IO;

namespace day10;

enum Instruction
{
    case NoOp;
    case AddX(int64 value);

    public void Display(UInt64 cycle)
    {
        switch (this) {
            case Instruction.NoOp:
                Console.WriteLine("{}: noop", cycle);
            case Instruction.AddX(let value):
                Console.WriteLine("{}: addx {}", cycle, value);
        }
    }
}

struct CPU
{
    struct QueuedInstruction
    {
        public Instruction instruction;
        public UInt64 cycle_added;

        public this(Instruction instruction, UInt64 cycle_added)
        {
            this.instruction = instruction;
            this.cycle_added = cycle_added;
        }

        public bool finished(UInt64 cycle)
        {
            switch (this.instruction)
            {
                case Instruction.NoOp:
                   return cycle > this.cycle_added;
                case Instruction.AddX:
                    return cycle > this.cycle_added + 1;
            }
        }
    }

    public int64 X;
    public UInt64 cycle;
    public int64 signal;
    public String screen;
    QueuedInstruction current_instruction;

    public this()
    {
        this.X = 1;
        this.cycle = 0;
        this.signal = 0;
        this.screen = new String();
        this.current_instruction = QueuedInstruction(Instruction.NoOp, 0);
    }

    /* Add a new instruction to be executed */
    public void queue_instruction(Instruction instruction) mut
    {
        this.current_instruction = QueuedInstruction(instruction, this.cycle);
    }

    /* Advance the cpu one cycle forward */
    public bool tick() mut
    {
        /* render screen */
        this.RenderPixel();
        /* increment cycle */
        this.cycle += 1;
        /* Add to signal if necessary */
        this.signal += this.GetSignal();
        /* Check if current job finished and apply result */
        if (this.current_instruction.finished(this.cycle))
        {
            if (this.current_instruction.instruction case Instruction.AddX(let value))
            {
                this.X += value;
            }
            return true;
        }
        return false;
    }

    /* Check if a signal should be given */
    public int64 GetSignal()
    {
        if (this.cycle == (UInt64) 20)
        {
            return (int64) this.cycle * this.X;
        } else if (this.cycle > (UInt64) 20){
            let adjusted = (UInt64)(this.cycle - 20);
            if (adjusted % (UInt64) 40 == 0)
            {
                return (int64) this.cycle * this.X;
            }
            else
            {
                return 0;
            }
        } else {
            return 0;
        }
    }

    /* Render a signal pixel to the screen */
    public void RenderPixel() mut
    {
        let pos = this.X;
        let pixel = (int64) (this.cycle % (UInt64) 40);
        if (pixel == 0)
        {
            this.screen += "\n";
        }
        if (pixel == pos - 1 || pixel == pos || pixel == pos + 1)
        {
            this.screen += "#";
        } else {
            this.screen += ".";
        }

    }
}

class Program
{
	public static int Main(String[] args)
	{
        run("/home/satan/Projects/aoc2022/day10/input.txt");
		return 0;
	}

    public static void run(StringView filename)
    {
        var contents = scope String();
        File.ReadAllText(filename, contents);
        var lines = contents.Split('\n');
        var instructions = new List<Instruction>();
        while (lines.GetNext() case .Ok(let line))
        {
            var ops = line.Split(' ');
            if (ops.GetNext() case .Ok(let op))
            {
                switch(op)
                {
                    case "noop":
                        instructions.Add(Instruction.NoOp);
                    case "addx":
                        if (ops.GetNext() case .Ok(let val))
                        {
                            instructions.Add(Instruction.AddX(int64.Parse(val)));
                        }
                    default:
                        continue;
                }
            }
        }
        var insts = instructions.GetEnumerator();
        var cpu = new CPU();
        while(insts.GetNext() case .Ok(let instruction))
        {
            cpu.queue_instruction(instruction);
            while (!cpu.tick())
            {
                continue;
            }
        }
        Console.WriteLine("Total: {}", cpu.signal);
        Console.WriteLine(cpu.screen);
    }
}