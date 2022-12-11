use std::collections::VecDeque;

use common::read_main;

fn solve(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|raw_monkey| {
            let mut raw_monkey = raw_monkey.lines().skip(1); // Monkey N
            let mut next_stripped = |prefix: &str| {
                raw_monkey
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix(prefix)
                    .unwrap()
            };
            let starting = next_stripped("Starting items: ");
            let worries = starting.split(", ").map(|w| w.parse().unwrap()).collect();

            let operation = next_stripped("Operation: new = old ");
            let operation: Box<dyn Fn(_) -> _> = match operation {
                "* old" => Box::new(|old| old * old),
                "+ old" => Box::new(|old| old + old),
                _ => {
                    let (operation, operand) = operation.split_once(' ').unwrap();
                    let operand: u32 = operand.parse().unwrap();
                    match operation {
                        "*" => Box::new(move |old| old * operand),
                        "+" => Box::new(move |old| old + operand),
                        op => panic!("unhandled operation: {op}"),
                    }
                }
            };

            let test: u32 = next_stripped("Test: divisible by ").parse().unwrap();
            let if_true = next_stripped("If true: throw to monkey ").parse().unwrap();
            let if_false = next_stripped("If false: throw to monkey ").parse().unwrap();
            let test = Box::new(
                move |worry| {
                    if worry % test == 0 {
                        if_true
                    } else {
                        if_false
                    }
                },
            );

            Monkey {
                worries,
                operation,
                test,
                num_inspections: 0,
            }
        })
        .collect();
    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|monkey_index| {
            while let Some(worry) = monkeys[monkey_index].inspect() {
                let worry = worry / 3;
                let throw_to = monkeys[monkey_index].test(worry);
                monkeys[throw_to].receive(worry);
            }
        })
    });

    monkeys.sort_by_key(|m| m.num_inspections);
    monkeys.pop().unwrap().num_inspections * monkeys.pop().unwrap().num_inspections
}

struct Monkey {
    worries: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> usize>,
    num_inspections: usize,
}
impl Monkey {
    fn inspect(&mut self) -> Option<u32> {
        self.worries.pop_front().map(|old| {
            self.num_inspections += 1;
            (self.operation)(old)
        })
    }

    fn test(&self, worry: u32) -> usize {
        (self.test)(worry)
    }

    fn receive(&mut self, worry: u32) {
        self.worries.push_back(worry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        assert_eq!(solve(input), 10605);
    }
}

read_main!();
