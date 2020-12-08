use std::collections::{HashMap, HashSet};

fn solve(instructions: &str) -> i32 {
    let instructions: Vec<_> = instructions
        .trim()
        .lines()
        .map(|l| {
            let mut instructions = l.split_whitespace();
            let opcode = instructions.next().unwrap();
            let mut arg = instructions.next().unwrap();
            if &arg[0..1] == "+" {
                arg = &arg[1..];
            }
            match opcode {
                "acc" => Instruction::Acc(arg.parse().unwrap()),
                "nop" => Instruction::Nop(arg.parse().unwrap()),
                "jmp" => Instruction::Jmp(arg.parse().unwrap()),
                o => panic!("unexpected opcode: {:?}", o),
            }
        })
        .collect();

    let jump_spots: HashMap<_, Vec<_>> = instructions.iter().enumerate().fold(
        HashMap::new(),
        |mut jump_spots, (index, instruction)| match instruction {
            Instruction::Jmp(jmp) if *jmp != 1 => {
                let target = (index as isize + jmp) as usize;
                jump_spots.entry(target).or_insert(vec![]).push(index);
                jump_spots
            }
            _ => jump_spots,
        },
    );

    let flipped_jump_spots: HashMap<_, Vec<_>> = instructions.iter().enumerate().fold(
        HashMap::new(),
        |mut flipped_jump_spots, (index, instruction)| match instruction {
            Instruction::Nop(jmp) if *jmp > 1 => {
                let target = (index as isize + jmp) as usize;
                flipped_jump_spots
                    .entry(target)
                    .or_insert(vec![])
                    .push(index);
                flipped_jump_spots
            }
            _ => flipped_jump_spots,
        },
    );

    let mut frontier = vec![Node {
        accumulator: 0,
        flipped: None,
        index: instructions.len(),
    }];

    let mut explored = HashSet::new();

    while let Some(Node {
        index,
        accumulator,
        flipped,
    }) = frontier.pop()
    {
        // maybe we are done
        if index == 0 {
            return accumulator;
        }

        explored.insert((index, flipped));

        let mut new_frontier = vec![];

        // maybe it came from the instruction before
        match instructions[index - 1] {
            Instruction::Nop(_) | Instruction::Jmp(1) => new_frontier.push(Node {
                index: index - 1,
                flipped,
                accumulator,
            }),
            Instruction::Acc(acc) => new_frontier.push(Node {
                index: index - 1,
                flipped,
                accumulator: accumulator + acc,
            }),
            Instruction::Jmp(_) if flipped.is_none() => new_frontier.push(Node {
                index: index - 1,
                flipped: Some(index - 1),
                accumulator,
            }),
            _ => {}
        }

        // or from a jump
        if let Some(sources) = jump_spots.get(&index) {
            new_frontier.extend(sources.iter().filter(|i| flipped != Some(**i)).map(
                |&source_index| Node {
                    index: source_index,
                    flipped,
                    accumulator,
                },
            ))
        }

        // or from a nop that was meant to be a jmp
        if flipped.is_none() {
            if let Some(sources) = flipped_jump_spots.get(&index) {
                new_frontier.extend(sources.iter().map(|&source_index| Node {
                    index: source_index,
                    flipped,
                    accumulator,
                }))
            }
        }

        frontier.extend(
            new_frontier
                .into_iter()
                .filter(|n| !explored.contains(&(n.index, n.flipped))),
        );
    }

    panic!("did not find path to beginning")
}

#[derive(Debug)]
struct Node {
    accumulator: i32,
    flipped: Option<usize>,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn example_two() {
        let input = r"nop +7
jmp +1
jmp -1
jmp +1
jmp -1
jmp +1
jmp -1
acc +6";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn example_three() {
        let input = r"acc +7
nop +7
jmp +1
jmp -1
jmp +1
jmp -1
jmp +1
jmp -1
nop +6";
        assert_eq!(solve(input), 7);
    }

    #[test]
    fn example_four() {
        let input = r"acc +7
jmp +12
jmp -1
nop +2
jmp -1
nop +2
jmp -1
nop +2
jmp -1
nop +2
jmp -1
nop +2
jmp -1
acc -3
jmp +1";
        assert_eq!(solve(input), 4);
    }
}

common::read_main!();
