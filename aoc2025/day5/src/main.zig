// zig 0.15.2
const std = @import("std");
const alloc = std.heap.page_allocator;
// read the input of the file at comptime
const input = @embedFile("input.txt");

const Range = struct {
    low: u64,
    high: u64,

    // check if number is in the inclusive range
    fn contains_inc(self: *const Range, num: u64) bool {
        return self.low <= num and num <= self.high;
    }

    // check if number is in range except the high end
    fn contains_lower_inc(self: *const Range, num: u64) bool {
        return self.low <= num and num < self.high;
    }

    // check if number is in range except the low end
    fn contains_upper_inc(self: *const Range, num: u64) bool {
        return self.low < num and num <= self.high;
    }
};

const Input = struct {
    ranges: std.ArrayList(Range),
    singletons: std.ArrayList(u64),
    inputs: std.ArrayList(u64),

    fn contains(self: *const Input, num: u64) bool {
        for (self.ranges.items) |r| {
            if (r.contains_inc(num)){
                return true;
            }
        }
        for (self.singletons.items) |r| {
            if (r == num){
                return true;
            }
        }
        return false;
    }

    fn part_one(self: *const Input) u64 {
        var count: u64 = 0;
        for (self.inputs.items) |id| {
            if (self.contains(id)) {
                count += 1;
            }
        }
        return count;
    }

    // remove unnecessary singletons
    fn prune_singletons(self: *Input) !void {
        std.mem.sort(u64, self.singletons.items, {}, std.sort.asc(u64));
        var pruned = try std.ArrayList(u64).initCapacity(alloc, self.singletons.items.len);
        for (self.singletons.items) |s| {
            var keep = true;
            for (self.ranges.items) |r| {
                if (r.contains_inc(s)){
                    keep = false;
                }
            }
            if (keep) {
                try pruned.append(alloc, s);
            }
        }
        // if all singletons pruned, return
        if (pruned.items.len == 0) {
            self.singletons = pruned;
            return;
        }
        // deduplicate those singletons that are left
        var pruned_dedup = try std.ArrayList(u64).initCapacity(alloc, self.singletons.items.len);
        std.mem.sort(u64, pruned.items, {}, std.sort.asc(u64));
        var last = pruned.items[0];
        try pruned_dedup.append(alloc, last);
        for (pruned.items[1..]) |elem| {
            if (elem != last) {
                try pruned_dedup.append(alloc, elem);
                last = elem;
            }
        }
        self.singletons = pruned_dedup;
    }

    // compute the endpoints of the union of the contained ranges
    fn endpoints(self: *const Input) !std.ArrayList(u64) {
        var ends = try std.ArrayList(u64).initCapacity(alloc, 2 * self.ranges.items.len);
        for (self.ranges.items, 0..) |r1, i| {
            // check if the low and high points are contained in the unique endpoints
            var low = true;
            var high = true;
            for (self.ranges.items, 0..) |r2, j| {
                // don't check a range against itself
                if (i != j) {
                    if (low and r2.contains_upper_inc(r1.low)) {
                        low = false;
                    }
                    if (high and r2.contains_lower_inc(r1.high)) {
                        high = false;
                    }
                    // short circuit
                    if (!low and !high) {
                        break;
                    }
                }
            }
            if (low) {
                try ends.append(alloc, r1.low);
            }
            if (high) {
                try ends.append(alloc, r1.high);
            }
        }
        // deduplicate endpoints. Can happen if two ranges share a high or share a low
        std.mem.sort(u64, ends.items, {}, std.sort.asc(u64));
        var ends_dedup = try std.ArrayList(u64).initCapacity(alloc, 2 * self.ranges.items.len);
        var last = ends.items[0];
        try ends_dedup.append(alloc, last);
        for (ends.items[1..]) |elem| {
            if (elem != last) {
                try ends_dedup.append(alloc, elem);
                last = elem;
            }
        }
        // add in singleton ranges
        for (self.singletons.items) |s| {
            try ends_dedup.append(alloc, s);
            try ends_dedup.append(alloc, s);
        }
        return ends_dedup;
    }
};


pub fn parse_input() !Input {
    var splits = std.mem.splitAny(u8, input, "\n");
    var parsing_ranges = true;
    var parsed = Input {
        .ranges = try std.ArrayList(Range).initCapacity(alloc, 4),
        .singletons = try std.ArrayList(u64).initCapacity(alloc, 4),
        .inputs = try std.ArrayList(u64).initCapacity(alloc, 8),
    };
    while (splits.next()) |line| {
        const trimmed = std.mem.trim(u8, line, "\n");
        if (trimmed.len == 0) {
            parsing_ranges = false;
            continue;
        }
        if (parsing_ranges) {
            var range_splits = std.mem.splitAny(u8, trimmed, "-");
            const low_str = range_splits.next().?;
            const high_str = range_splits.next().?;
            const low = try std.fmt.parseInt(u64, low_str, 10);
            const high = try std.fmt.parseInt(u64, high_str, 10);
            if (low == high) {
                try parsed.singletons.append(alloc, low);
            } else {
                try parsed.ranges.append(alloc, Range{ .low = low, .high = high });
            }
        } else {
            const input_id = try std.fmt.parseInt(u64, trimmed, 10);
            try parsed.inputs.append(alloc, input_id);
        }

    }
    // remove unnecessary singletons here
    try parsed.prune_singletons();
    return parsed;
}

pub fn part_one() !void {
    const inputs = try parse_input();
    const res = inputs.part_one();
    std.debug.print("Part one: {d}\n", .{res});
}

pub fn part_two() !void {
    const inputs = try parse_input();
    const ends = try inputs.endpoints();
    const len = ends.items.len;

    var valid_id_count: u64 = 0;
    for (0..(len  / 2 )) |i| {
        const elems = ends.items[2*i+1] - ends.items[2*i] + 1;
        valid_id_count += elems;
    }
    std.debug.print("Part two: {d}\n", .{valid_id_count});
}

pub fn main() !void {
    try part_one();
    try part_two();
}
