use std::collections::VecDeque;


struct Monkey
{
    pub items: VecDeque<u64>,
    pub op: Box<dyn Fn(u64) -> u64>,
    pub test: Box<dyn Fn(u64) -> bool>,
    pub false_recipient: usize,
    pub true_recipient: usize,
    pub inspections: u64,
}

struct Throw {
    recipient: usize,
    item: u64,
}

impl Monkey {
    pub fn inspect(&mut self) -> Option<Throw> {
        self.items
            .pop_front()
            .map(|item|{
                self.inspections += 1;
                let item = (self.op)(item).checked_rem(9699690).unwrap();

                Throw {
                    recipient: if (self.test)(item) {
                        self.true_recipient
                    } else {
                        self.false_recipient
                    },
                    item,
                }
            })
    }
}

fn monkey_business(mut monkeys: [Monkey; 8]) -> u64 {
    // 20 rounds of passing
    for _ in 0..10000 {
        // iterate over each monkey each round
        for i in 0..8 {
            // check if something is thrown
            while let Some(throw) = monkeys[i].inspect() {
                // send thrown object to recipient
                monkeys
                    .get_mut(throw.recipient)
                    .map(|m| m.items.push_back(throw.item));
            }
        }
    }
    // Find top two number of inspections
    let (which, max) = monkeys
        .iter()
        .enumerate()
        .map(| (num, monkey)| (num, monkey.inspections))
        .max_by_key(|(_, inspections)| *inspections )
        .unwrap();
    let penult = monkeys
        .iter()
        .enumerate()
        .filter_map(| (num, monkey)| if num != which {
            Some(monkey.inspections)
        } else {
            None
        })
        .max()
        .unwrap();
    penult * max
}



fn main() {
    let monkeys: [Monkey; 8] = [
        Monkey {
            items: VecDeque::from([99, 63, 76, 93, 54, 73]),
            op: Box::new(|old| old * 11),
            test: Box::new(|x| x.checked_rem(2).unwrap() == 0),
            false_recipient: 1,
            true_recipient: 7,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([91, 60, 97, 54]),
            op: Box::new(|old| old + 1),
            test: Box::new(|x| x.checked_rem(17).unwrap() == 0),
            false_recipient: 2,
            true_recipient: 3,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([65]),
            op: Box::new(|old| old + 7),
            test: Box::new(|x| x.checked_rem(7).unwrap() == 0),
            false_recipient: 5,
            true_recipient: 6,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([84, 55]),
            op: Box::new(|old| old + 3),
            test: Box::new(|x| x.checked_rem(11).unwrap() == 0),
            false_recipient: 6,
            true_recipient: 2,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([86, 63, 79, 54, 83]),
            op: Box::new(|old| old * old),
            test: Box::new(|x| x.checked_rem(19).unwrap() == 0),
            false_recipient: 0,
            true_recipient: 7,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([96, 67, 56, 95, 64, 69, 96]),
            op: Box::new(|old| old + 4),
            test: Box::new(|x| x.checked_rem(5).unwrap() == 0),
            false_recipient: 0,
            true_recipient: 4,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([66, 94, 70, 93, 72, 67, 88, 51]),
            op: Box::new(|old| old * 5),
            test: Box::new(|x| x.checked_rem(13).unwrap() == 0),
            false_recipient: 5,
            true_recipient: 4,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([59, 59, 74]),
            op: Box::new(|old| old + 8),
            test: Box::new(|x| x.checked_rem(3).unwrap() == 0),
            false_recipient: 3,
            true_recipient: 1,
            inspections: 0,
        },

    ];
    let score = monkey_business(monkeys);

    println!("Part one: {}", score);
}
