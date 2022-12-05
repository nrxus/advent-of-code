use std::str::FromStr;

use common::read_main;

fn solve(input: &str) -> String {
    let (raw_stacks, movements) = input.trim_end().split_once("\n\n").unwrap();
    let mut raw_stacks = raw_stacks.lines().rev();
    // the last line is stack numbers; we just need to know how many
    // there are total so grab the last one
    let num_stacks: usize = raw_stacks
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];
    for line in raw_stacks {
        let line = line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            // my emacs is setup to eat up trailing spaces and ain't
            // nobody got time to figure out how to disable that; we
            // need to bound-check
            if let Some(&b) = line.get(4 * i + 1).filter(|c| **c != b' ') {
                stack.push(b as char)
            }
        }
    }

    movements
        .lines()
        .map(|m| Movement::from_str(m).unwrap())
        .for_each(|m| {
            let from = &mut stacks[m.from - 1];
            // collect it to appease the mighty borrow checker
            let drained: Vec<_> = from.drain(from.len() - m.num..).rev().collect();
            let to = &mut stacks[m.to - 1];
            to.extend(drained);
        });

    stacks
        .into_iter()
        .map(|s| s.last().copied().unwrap())
        .collect()
}

#[derive(Debug)]
struct Movement {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Movement {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        s.next().unwrap(); // move
        let num = s.next().unwrap().parse()?;
        s.next().unwrap(); // from
        let from = s.next().unwrap().parse()?;
        s.next().unwrap(); // to
        let to = s.next().unwrap().parse()?;
        Ok(Self { num, from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

        assert_eq!(solve(input), "CMZ".to_string());
    }
}

read_main!();
