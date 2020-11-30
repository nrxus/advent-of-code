mod guard_schedule;

use self::guard_schedule::*;

use std::{collections::HashMap, convert::TryFrom};

fn solve(input: &str) -> u16 {
    let mut entries: Vec<_> = input.lines().map(|l| Entry::try_from(l).unwrap()).collect();
    entries.sort_unstable_by(|a, b| a.date().cmp(b.date()));
    let guards = Schedule::guard_schedules(&entries).unwrap();

    let (id, schedules) = guards
        .iter()
        .max_by_key(|(_, schedules)| {
            schedules
                .iter()
                .flat_map(|s| s.minutes.iter())
                .filter(|&&s| s == GuardState::Asleep)
                .count()
        })
        .unwrap();

    let minute = schedules
        .iter()
        .flat_map(Schedule::asleep_minutes)
        .fold(HashMap::<usize, u16>::new(), |mut acc, m| {
            let count = acc.entry(m).or_default();
            *count += 1;
            acc
        })
        .iter()
        .max_by_key(|(_, &c)| c)
        .map(|(m, _)| m)
        .cloned()
        .unwrap();

    id * minute as u16
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(solve(input), 240);
    }
}

common::bootstrap!(4);
