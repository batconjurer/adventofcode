// zig 0.15.2
const std = @import("std");
const alloc = std.heap.page_allocator;
// read the input of the file at comptime
const input = @embedFile("input.txt");


const Grid = struct {
    inner: std.ArrayList(std.ArrayList(bool)),
    start: [2]u64,
};


fn parse_input() !Grid {
    var lines = std.mem.splitAny(u8, input, "\n");
    var grid = try std.ArrayList(std.ArrayList(bool)).initCapacity(alloc, 143);
    var start = [2]u64{0 , 0};


    while (lines.next()) |line| {
        var row = try std.ArrayList(bool).initCapacity(alloc, line.len);
        for (line, 0..) |c, col| {
            try row.append(alloc, c == '^');
            if (c == 'S') {
                start = [2]u64{0, col};
            }
        }

        try grid.append(alloc, row);
    }

    return Grid{.inner = grid, .start = start};
}


fn add_beams(
    beams: *std.AutoArrayHashMap(u64, u64),
    key: u64,
    value: u64
) !void {
    const previous_value = beams.get(key);
    if (previous_value != null) {
        try beams.put(key, previous_value.? + value);
    } else {
        try beams.put(key, value);
    }
}

pub fn main() !void {
    const grid = try parse_input();

    // for each row, count the number of beams in each col
    var beams = std.AutoArrayHashMap(u64, u64).init(alloc);
    try beams.put(grid.start[1], 1);


    var splits: u64 = 0;
    for (grid.inner.items) | row| {
        var new_beams = std.AutoArrayHashMap(u64, u64).init(alloc);
        for (row.items, 0..) |cell, col_num| {
            // check if a beam splits
            const num_beams = beams.get(col_num);
            if ( cell and num_beams != null) {
                splits += 1;
                if (col_num > 0) {
                    try add_beams(&new_beams, col_num - 1, num_beams.?);
                }
                if (col_num < row.items.len - 1) {
                    try add_beams(&new_beams, col_num + 1, num_beams.?);
                }
            } else if (num_beams != null) {
                // otherwise beams travels forward
                try add_beams(&new_beams, col_num, num_beams.?);
            }
        }
        beams = new_beams;
    }
    std.debug.print("Part one: {d}\n", .{splits});
    var iter = beams.iterator();
    var timelines: u64 = 0;
    while (iter.next()) |bs| {
        timelines += bs.value_ptr.*;
    }
    std.debug.print("Part two: {d}\n", .{timelines});
}