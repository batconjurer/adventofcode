import std.stdio : File, stdout, writeln, writefln;
import std.algorithm.iteration: splitter, map;
import std.conv: to;
import std.string: strip;
import std.range.primitives;
import std.array;
import std.algorithm;

struct OrderRule {
    uint before;
    uint after;
}

struct Updates {
    uint[] inner;
}

OrderRule[1176] parse_order(string filename) {
    auto file = File(filename, "r");
    string line;
    OrderRule[1176] ordering;
    uint counter = 0;
    while ((line = file.readln()) !is null) {
        auto nums = line.strip().splitter('|');
        auto before = to!uint(nums.front);
        nums.popFront();
        auto after = to!uint(nums.front);
        ordering[counter] = OrderRule(before, after);
        counter += 1;
    }
    return ordering;
}

void part_1(string orders_file, string updates_filename) {
    auto rules = parse_order(orders_file);
    auto file = File(updates_filename, "r");
    string line;
    uint sum = 0;
    while ((line = file.readln()) !is null) {
        bool is_valid = true;
        auto update = line.strip().splitter(',').map!( a => to!uint(a));
        foreach(page; update) {
            foreach(ref rule; rules) {
                // check if this rule applies to this page
                if (rule.before == page) {
                    // check all pages prior to this one
                    foreach(other; update) {
                        // if we have reached the current page without
                        // violating validity, stop
                        if (page == other) {
                            break;
                        }
                        // if a rule gets broken, mark update as invalid
                        // and stop
                        if (other == rule.after) {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if (is_valid) {
            auto mid_ix = update.walkLength() / 2;
            update.popFrontN(mid_ix);
            sum += update.front;
        }
    }
    writefln("Part 1: %d", sum);
}

void part_2(string orders_file, string updates_filename) {
    auto file = File(orders_file, "r");
    string line;
    int[uint[2]] ordering;

    while ((line = file.readln()) !is null) {
        auto nums = line.strip().splitter('|');
        auto before = to!uint(nums.front);
        nums.popFront();
        auto after = to!uint(nums.front);
        ordering[[before, after]] = 0;
    }

    file = File(updates_filename, "r");
    uint sum = 0;
    alias myComp = (x, y) => ([x,y] in ordering) !is null;
    while ((line = file.readln()) !is null) {
        auto update = array(line.strip().splitter(',').map!( a => to!uint(a)));
        auto mid_ix = update.walkLength() / 2;
        auto sorted = update.sort!(myComp, SwapStrategy.stable);
        sorted.popFrontN(mid_ix);
        sum += sorted.front;
    }
    // value from part 1
    sum -= 6034;
    writefln("Part 2: %d", sum);
}


void main() {
    part_1("input_order.txt", "input_updates.txt");
    part_2("input_order.txt", "input_updates.txt");
}