use std::collections::HashMap;

fn split_digits(mut num: u64) -> Option<(u64, u64)> {
    let mut length = num.checked_ilog10()? + 1;
    if length & 1 == 1 {
        return None;
    }
    let mut second = 0u64;
    let mut second_len = 0u32;
    while second_len != length {
        second += num.rem_euclid(10) * 10_u64.pow(second_len);
        second_len += 1;
        num /= 10;
        length -= 1;
    }
    Some((num, second))
}

fn blink(rock: u64) -> [Option<u64>; 2] {
    if rock == 0 {
        [Some(1), None]
    } else if let Some((f, s)) = split_digits(rock) {
        [Some(f), Some(s)]
    } else {
        [Some(rock * 2024), None]
    }
}

/// Compute a map saying how many of each rock is produced after
/// `depth` blinks. Multiplier is used to calculate the effect of
/// `multiplier` rocks simultaneously.
fn dfs(
    rock: &u64,
    depth: u8,
    res: &mut HashMap<u64, u64>,
    multiplier: u64,
) {
    let mut stack = Vec::from([(*rock, 0u8)]);
    while let Some((next, level)) = stack.pop() {
        if level == depth {
            {
                res.entry(next)
                    .and_modify(|x| *x += multiplier)
                    .or_insert(multiplier);
            }
            continue;
        }

        let ns = blink(next);
        for n in ns.into_iter().filter_map(|x| x) {
            stack.push((n, level + 1))
        }
    }
}

fn part_1(rocks: Vec<u64>) {
    let mut res =  HashMap::<u64, u64>::new();
    for rock in rocks {
        dfs(&rock, 25, &mut res, 1);

    }

    let sum: u64 = res.values().cloned().sum();
    println!("Part 1 : {sum}" );
}

fn part_2(rocks: Vec<u64>) {
    let mut res =  HashMap::<u64, u64>::new();
    for rock in rocks {
        dfs(&rock, 25, &mut res, 1);

    }
    for _ in 0..10 {
        let mut res_new = HashMap::<u64, u64>::new();
        for (rock, multiplier) in &res {
            dfs(&rock, 5, &mut res_new, *multiplier);
        }
        std::mem::swap(&mut res, &mut res_new);
    }

    let sum: u64 = res.values().cloned().sum();
    println!("Part 2 : {sum}" );
}

fn main() {
    part_1( vec![6563348, 67, 395, 0, 6, 4425, 89567, 739318]);
    part_2( vec![6563348, 67, 395, 0, 6, 4425, 89567, 739318]);
}
