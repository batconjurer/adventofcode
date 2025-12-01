import std.stdio : File, stdout, write, writeln, writefln;
import std.algorithm.iteration: splitter, map;
import std.conv: to;
import std.string: strip;
import std.range.primitives;
import std.array;
import std.algorithm;

enum SIZE = 130;
enum Obstruction { None, Some }
enum Dir {Up, Left, Down, Right}
enum Action {Free, Move, Turn, Loop}

Dir turn(Dir dir) {
    switch(dir) {
        case Dir.Up:
            dir = Dir.Right;
            break;
        case Dir.Right:
            dir = Dir.Down;
            break;
        case Dir.Down:
            dir = Dir.Left;
            break;
        case Dir.Left:
            dir = Dir.Up;
            break;
        default: assert(0);
    }
    return dir;
}

struct Grid {
    uint[2] pos;
    uint[2] original_pos;
    Dir direction;
    Obstruction[SIZE][SIZE] grid;
    Dir[uint[2]] visited;
}

void reset(Grid* grid, uint[2] obs) {
    grid.visited.clear;
    grid.pos = grid.original_pos;
    grid.direction = Dir.Up;
    grid.grid[obs[0]][obs[1]] = Obstruction.None;
}

void display(Grid grid) {
    for (uint i=0; i<SIZE; i++) {
        for (uint j=0; j<SIZE; j++) {
            if (([i, j] in grid.visited) !is null) {
                write("X");
            } else {
                switch(grid.grid[i][j]) {
                    case Obstruction.None:
                        write(".");
                        break;
                    default:
                        write("#");
                }
            }
        }
        write("\n");
    }
}

Action step(Grid* grid) {
    uint[2] new_pos;
    switch(grid.direction) {
        case Dir.Up:
            if (grid.pos[0] > 0) {
                new_pos = [grid.pos[0] - 1, grid.pos[1]];
            } else {
                return Action.Free;
            }
            break;
        case Dir.Right:
            if (grid.pos[1] < SIZE - 1) {
                new_pos = [grid.pos[0], grid.pos[1] + 1];
            } else {
                return Action.Free;
            }
            break;
        case Dir.Down:
            if (grid.pos[0] < SIZE - 1) {
                new_pos = [grid.pos[0] + 1, grid.pos[1]];
            } else {
                return Action.Free;
            }
            break;
        case Dir.Left:
            if (grid.pos[1] > 0) {
                new_pos = [grid.pos[0], grid.pos[1] - 1];
            } else {
                return Action.Free;
            }
            break;
        default: assert(0);
    }


    if (grid.grid[new_pos[0]][new_pos[1]] == Obstruction.None) {
        auto dir = new_pos in grid.visited;
        if ( dir !is null && *dir == grid.direction) {
            return Action.Loop;
        } else {
            grid.visited[new_pos] = grid.direction;
            grid.pos = new_pos;
            return Action.Move;
        }
    } else {
        grid.direction = grid.direction.turn();
        return Action.Turn;
    }
}

Grid parse(string filename) {
    auto file = File(filename, "r");
    string line;
    auto grid = Grid([0, 0], Dir.Up);
    uint row = 0;
    while ((line = file.readln()) !is null) {
        auto chars = line.strip();
        uint col = 0;
        foreach(c; chars) {
            Obstruction obs;
            switch(c) {
                case '.':
                    obs = Obstruction.None;
                    break;
                case '#':
                    obs = Obstruction.Some;
                    break;
                case '^':
                    obs = Obstruction.None;
                    grid.pos = [row, col];
                    grid.original_pos = [row, col];
                    grid.visited[[row, col]] = Dir.Up;
                    break;
                default:
                    break;
            }
            grid.grid[row][col] = obs;
            col += 1;
        }
        row += 1;
    }
    return grid;
}

void part_1(string filename) {
    auto grid = parse(filename);
    bool finished = false;
    while (!finished) {
        switch (step(&grid)) {
            case Action.Free:
                finished = true;
                break;
            default:
                continue;
        }
    }
    writefln("Part 1: %d", grid.visited.length);
}

void part_2(string filename) {
    auto grid = parse(filename);
    uint counter = 0;
    for (uint i=0; i < SIZE; i++) {
        for (uint j=0; j < SIZE; j++) {
            if (grid.grid[i][j] == Obstruction.Some || [i, j] == grid.pos) {
                continue;
            } else {
                grid.grid[i][j] = Obstruction.Some;
            }
            bool finished = false;
            while (!finished) {
                switch (step(&grid)) {
                    case Action.Free:
                        finished = true;
                        break;
                    case Action.Loop:
                        counter += 1;
                        finished = true;
                        break;
                    default:
                        continue;
                }
            }
            reset(&grid, [i, j]);
        }
    }
    writefln("Part 2: %d", counter);
}

void main () {
    part_1("input.txt");
    part_2("input.txt");
}