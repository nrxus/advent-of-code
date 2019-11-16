use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> u32 {
    winner_distance_after(input, 2503)
}

fn winner_distance_after(input: &str, time: u32) -> u32 {
    input
        .lines()
        .map(Reindeer::from)
        .map(|r| r.travel(time))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Reindeer<'s> {
    name: &'s str,
    fly_duration: u32,
    rest_duration: u32,
    speed: u32,
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

impl Reindeer<'_> {
    fn travel(&self, duration: u32) -> u32 {
        let single_flight_duration = self.fly_duration + self.rest_duration;
        let full_flights = duration / single_flight_duration;
        let remaining_duration = duration % single_flight_duration;
        let single_flight_length = self.fly_duration * self.speed;

        full_flights * single_flight_length
            + if self.fly_duration > remaining_duration {
                remaining_duration * self.speed
            } else {
                single_flight_length
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
        assert_eq!(winner_distance_after(input, 1000), 1120);
    }
}

common::read_main!();
