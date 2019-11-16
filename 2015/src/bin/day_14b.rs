use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    winner_points_after(input, 2503)
}

fn winner_points_after(input: &str, time: u32) -> u32 {
    let mut reindeers: Vec<_> = input
        .lines()
        .map(Reindeer::from)
        .map(RunningReindeer::new)
        .collect();

    let mut tally: HashMap<&str, u32> = HashMap::new();

    (0..time).for_each(|_| {
        reindeers.iter_mut().for_each(|r| r.next());
        let max_position = reindeers.iter().map(|r| r.position).max().unwrap();
        reindeers
            .iter()
            .filter(|r| r.position == max_position)
            .for_each(|r| {
                let points = tally.entry(r.reindeer.name).or_default();
                *points += 1;
            });
    });

    tally.values().max().unwrap().clone()
}

#[derive(Debug)]
struct Reindeer<'s> {
    name: &'s str,
    fly_duration: u32,
    rest_duration: u32,
    speed: u32,
}

struct RunningReindeer<'s> {
    second: u32,
    position: u32,
    reindeer: Reindeer<'s>,
}

impl<'s> RunningReindeer<'s> {
    fn new(reindeer: Reindeer<'s>) -> Self {
        RunningReindeer {
            reindeer,
            second: 0,
            position: 0,
        }
    }

    fn next(&mut self) {
        if self.second < self.reindeer.fly_duration {
            self.position += self.reindeer.speed;
            self.second += 1;
        } else {
            self.second += 1;
            self.second = self.second % (self.reindeer.fly_duration + self.reindeer.rest_duration);
        }
    }
}

impl<'s> From<&'s str> for Reindeer<'s> {
    fn from(input: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<name>[^\s]+) can fly (?P<speed>\d+) km/s for (?P<fly_duration>\d+) seconds, but then must rest for (?P<rest_duration>\d+) seconds.").unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let name = captures.name("name").unwrap().as_str();
        let speed: u32 = captures.name("speed").unwrap().as_str().parse().unwrap();
        let fly_duration: u32 = captures
            .name("fly_duration")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let rest_duration: u32 = captures
            .name("rest_duration")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        Reindeer {
            name,
            fly_duration,
            rest_duration,
            speed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";
        assert_eq!(winner_points_after(input, 1000), 689);
    }
}

common::read_main!();
