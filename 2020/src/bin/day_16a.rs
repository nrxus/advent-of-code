use regex::Regex;

fn solve(input: &str) -> u32 {
    let mut input = input.trim().split("\n\n");
    let rules = input.next().unwrap();
    let _ = input.next().unwrap();
    let nearby = input.next().unwrap();

    let constraint_regex =
        Regex::new(r".*: (?P<min_one>\d+)-(?P<max_one>\d+) or (?P<min_two>\d+)-(?P<max_two>\d+)")
            .unwrap();

    let constraints: Vec<_> = rules
        .lines()
        .into_iter()
        .flat_map(|rule| {
            let captures = constraint_regex.captures(rule).unwrap();
            let min_one = captures.name("min_one").unwrap().as_str().parse().unwrap();
            let max_one = captures.name("max_one").unwrap().as_str().parse().unwrap();
            let min_two = captures.name("min_two").unwrap().as_str().parse().unwrap();
            let max_two = captures.name("max_two").unwrap().as_str().parse().unwrap();
            vec![
                Constraint {
                    min: min_one,
                    max: max_one,
                },
                Constraint {
                    min: min_two,
                    max: max_two,
                },
            ]
        })
        .collect();

    nearby
        .lines()
        .skip(1)
        .flat_map(|n| n.split(','))
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|&n| constraints.iter().all(|c| !c.meets(n)))
        .sum()
}

#[derive(Debug)]
struct Constraint {
    min: u32,
    max: u32,
}

impl Constraint {
    fn meets(&self, n: u32) -> bool {
        n >= self.min && n <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(solve(input), 71);
    }
}

common::read_main!();
