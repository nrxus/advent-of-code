use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn solve(input: &str) -> i32 {
    let regex = Regex::new(r"(?P<guest_a>[^\s]+) would (?P<gain_or_lose>[^\s]+) (?P<happiness>[\d]+) happiness units by sitting next to (?P<guest_b>[^\s]+).").unwrap();

    let sittings = input
        .lines()
        .map(|l| {
            let captures = regex.captures(l).unwrap();
            let guest_a = captures.name("guest_a").unwrap().as_str();
            let guest_b = captures.name("guest_b").unwrap().as_str();
            let pair = GuestPair::new(guest_a, guest_b);
            let happiness: u32 = captures
                .name("happiness")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let happiness: i32 = if captures.name("gain_or_lose").unwrap().as_str() == "gain" {
                happiness as i32
            } else {
                -(happiness as i32)
            };
            (pair, happiness)
        })
        .fold(HashMap::new(), |mut sittings, (pair, happiness)| {
            let h = sittings.entry(pair).or_insert(0);
            *h += happiness;
            sittings
        });

    let mut guests: HashSet<_> = sittings.keys().flat_map(|p| vec![p.a, p.b]).collect();
    guests.insert("*");

    let mut queue: BinaryHeap<_> = sittings
        .iter()
        .map(|(pair, &happiness)| {
            let mut left_to_sit = guests.clone();
            left_to_sit.remove(pair.a);
            left_to_sit.remove(pair.b);
            Table {
                first_sat: pair.a,
                last_sat: pair.b,
                left_to_sit,
                happiness,
            }
        })
        .collect();

    let mut max_happiness = 0;

    while let Some(table) = queue.pop() {
        if table.left_to_sit.is_empty() {
            let last_pair = GuestPair::new(table.last_sat, table.first_sat);
            let new_happiness = table.happiness + sittings.get(&last_pair).unwrap_or(&0);
            max_happiness = std::cmp::max(max_happiness, new_happiness);
        }

        let table_arrangements = table.left_to_sit.iter().map(|guest| {
            let mut left_to_sit = table.left_to_sit.clone();
            left_to_sit.remove(guest);
            let new_pair = GuestPair::new(table.last_sat, guest);

            Table {
                left_to_sit,
                happiness: table.happiness + sittings.get(&new_pair).unwrap_or(&0),
                last_sat: guest,
                first_sat: table.first_sat,
            }
        });

        queue.extend(table_arrangements);
    }

    max_happiness
}

#[derive(Eq, Debug)]
struct Table<'s> {
    left_to_sit: HashSet<&'s str>,
    first_sat: &'s str,
    last_sat: &'s str,
    happiness: i32,
}

impl<'s> PartialEq for Table<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.happiness == other.happiness
    }
}

impl<'s> PartialOrd for Table<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'s> Ord for Table<'s> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.happiness.cmp(&other.happiness)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GuestPair<'s> {
    a: &'s str,
    b: &'s str,
}

impl<'s> GuestPair<'s> {
    fn new(a: &'s str, b: &'s str) -> Self {
        if a < b {
            GuestPair { a, b }
        } else {
            GuestPair { a: b, b: a }
        }
    }
}

common::read_main!();
