use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn solve(input: &str) -> usize {
    let mut grid = bit_vec::BitVec::from_elem(1000 * 1000, false);
    input
        .lines()
        .map(|l| Step::from_str(l).unwrap())
        .for_each(|step| {
            let range = (step.start.1..=step.end.1)
                .flat_map(|y| {
                    (step.start.0..=step.end.0).map(move |x| y as usize * 1000 + x as usize)
                })
                .collect::<Vec<_>>();
            match step.action {
                Action::Toggle => {
                    for i in range {
                        grid.set(i, !grid[i]);
                    }
                }
                Action::TurnOn => {
                    for i in range {
                        grid.set(i, true);
                    }
                }
                Action::TurnOff => {
                    for i in range {
                        grid.set(i, false);
                    }
                }
            }
        });
    grid.iter().filter(|x| *x).count()
}

struct Step {
    action: Action,
    start: (u16, u16),
    end: (u16, u16),
}

enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

impl FromStr for Step {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
            Regex::new(r"(?P<action>(?:turn on)|(?:turn off)|toggle) (?P<x_start>\d+),(?P<y_start>\d+) through (?P<x_end>\d+),(?P<y_end>\d+)").unwrap();
        }

        let captures = RE.captures(input).ok_or_else(|| "did not match regex")?;
        let action = captures
            .name("action")
            .ok_or_else(|| "failed to get action")?
            .as_str();
        let x_start = captures
            .name("x_start")
            .ok_or_else(|| "failed to get x_start")?
            .as_str()
            .parse::<u16>()?;
        let y_start = captures
            .name("y_start")
            .ok_or_else(|| "failed to get y_start")?
            .as_str()
            .parse::<u16>()?;
        let x_end = captures
            .name("x_end")
            .ok_or_else(|| "failed to get x_end")?
            .as_str()
            .parse::<u16>()?;
        let y_end = captures
            .name("y_end")
            .ok_or_else(|| "failed to get y_end")?
            .as_str()
            .parse::<u16>()?;
        let action = match action {
            "toggle" => Action::Toggle,
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            _ => return Err("invalid action".into()),
        };
        Ok(Step {
            action: action,
            start: (x_start, y_start),
            end: (x_end, y_end),
        })
    }
}

#[cfg(test)]
mod six_a {
    use super::*;

    #[test]
    fn test() {
        let input = r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        assert_eq!(solve(input), 998_996);
    }
}

common::read_main!();
