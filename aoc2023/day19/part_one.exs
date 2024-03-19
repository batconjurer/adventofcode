defmodule Check do
    defstruct [:name, :op, :const, :target]
    @type name :: :x | :m | :a | :s
    @type op :: :lt | :gt
    @type const :: integer()
    @type target ::  String.t() | :R |:A
    @type t :: %__MODULE__ {
        name: name(),
        op: op(),
        const: const(),
        target: target(),
    }
end

defmodule Rule do
    defstruct [:ty]
    @type ty :: {:default, String.t()| :R | :A} | {:check, Check.t()}

     def parse(line) do
        cond do
            String.contains?(line, "<") ->
                [name, rest] = String.split(line, "<")
                name = case name do
                    "x" -> :x
                    "m" -> :m
                    "a" -> :a
                    "s" -> :s
                end
                [const, target] = String.split(rest, ":")
                const = String.to_integer(const)
                target = case target do
                    "A" -> :A
                    "R" -> :R
                    _ -> target
                end
                %Rule{ty: %{check: %Check{name: name, op: :lt, const: const, target: target}}}
            String.contains?(line, ">") ->
                [name, rest] = String.split(line, ">")
                name = case name do
                    "x" -> :x
                    "m" -> :m
                    "a" -> :a
                    "s" -> :s
                end
                [const, target] = String.split(rest, ":")
                const = String.to_integer(const)
                target = case target do
                    "A" -> :A
                    "R" -> :R
                    _ -> target
                end
                %Rule{ty: %{check: %Check{name: name, op: :gt, const: const, target: target}}}
            1 -> %Rule{ty: %{default: case line do
                "A" -> :A
                "R" -> :R
                _ -> line
            end}}
        end
    end

    def apply(%Rule{ty: ty} = _, part) do
        case ty do
            %{default: target} -> {true, target}
            %{check: check} ->
                attr = Map.get(part, check.name)
                const = check.const
                case check.op do
                    :gt -> {attr > const, check.target}
                    :lt -> {attr < const, check.target}
                end
        end
    end
end


defmodule Workflow do
    defstruct [:name, :rules]
    @type name :: String
    @type rules :: [Rule]

    def parse(line) do
        line = String.replace(line, "}", "")
        [name , rules ] = String.split(line, "{")
        rules = Enum.map(String.split(rules, ","), fn(r) -> Rule.parse(r) end)
        %Workflow{name: name, rules: rules}
    end

    def apply(%Workflow{name: name, rules: rules}, part) do
        [rule | rules] = rules
        case Rule.apply(rule, part) do
            {true, target} -> target
            _ -> Workflow.apply(%Workflow{name: name, rules: rules}, part)
        end
    end

end

defmodule DayNineteen do
    def parse(w_file, p_file) do
        contents = File.read!(w_file)
        lines = String.split(contents, "\n")
        {_, workflows} = Enum.map_reduce(lines, %{}, fn(w, acc) ->
            workflow = Workflow.parse(w)
            acc = Map.put(acc, workflow.name, workflow)
            {workflow.name, acc}
         end)
        contents = File.read!(p_file)
        lines = String.split(contents, "\n")
        parts = Enum.map(lines, fn(p) -> parse_part(p) end)
        {workflows, parts}
    end

    def parse_part(line) do
        line = String.replace(line, "{", "")
        line = String.replace(line, "}", "")
        {_, part} = Enum.map_reduce(String.split(line, ","), %{}, fn(attr, acc) ->
            [attr, val] = String.split(attr, "=")
            val = String.to_integer(val)
            acc = case attr do
                "x" -> Map.put(acc, :x, val)
                "m" -> Map.put(acc, :m, val)
                "a" -> Map.put(acc, :a, val)
                "s" -> Map.put(acc, :s, val)
            end
            {attr, acc}
        end)
        part
    end

    def part_one(w_file, p_file) do
        {workflows, parts} = parse(w_file, p_file)
        {_, accepted} = Enum.map_reduce(parts, [], fn(p, acc) -> {
            p, apply_workflows(workflows, "in", p, acc)}
        end)
        {_, res} = Enum.map_reduce(accepted, 0, fn(part, acc) ->
            {part, acc + Map.get(part, :x)+ Map.get(part, :m)+ Map.get(part, :a)+ Map.get(part, :s)}
        end)
        IO.puts("Part one: #{res}")
    end

    def apply_workflow(workflow, part, acc) do
        case Workflow.apply(workflow, part) do
           :A -> {:done, List.insert_at(acc, 0, part)}
           :R -> {:done, acc}
           target -> {:next, target, acc}
        end
    end

    def apply_workflows(workflows, wf, part, acc) do
        workflow = Map.get(workflows, wf)
        case apply_workflow(workflow, part, acc) do
            {:done, acc} -> acc
            {:next, target, acc} -> apply_workflows(workflows, target, part, acc)
        end
    end
end

DayNineteen.part_one("input.txt", "parts.txt")
