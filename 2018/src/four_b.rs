#![feature(try_trait)]

mod guard_schedule;

use self::guard_schedule::*;
use std::{collections::HashMap, convert::TryFrom};

fn solve(input: &str) -> u32 {
    let mut entries: Vec<_> = input.lines().map(|l| Entry::try_from(l).unwrap()).collect();
    entries.sort_unstable_by(|a, b| a.date().cmp(b.date()));

    Schedule::guard_schedules(&entries)
        .unwrap()
        .into_iter()
        .filter_map(|(id, schedules)| {
            schedules
                .iter()
                .flat_map(Schedule::asleep_minutes)
                .fold(HashMap::<usize, u16>::new(), |mut acc, m| {
                    *(acc.entry(m).or_default()) += 1;
                    acc
                })
                .iter()
                .max_by_key(|(_, &c)| c)
                .map(|(&a, &b)| (a, b))
                .map(|t| (id, t))
        })
        .max_by_key(|(_, (_, count))| *count)
        .map(|(id, (minute, _))| u32::from(id) * minute as u32)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(solve(input), 4455);
    }
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let output = solve(&input);
    println!("{}", output);
}

//common::bootstrap!(4);
