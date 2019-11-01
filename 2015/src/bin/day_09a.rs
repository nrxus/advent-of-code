use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{Ordering, PartialOrd},
    collections::{BinaryHeap, HashMap, HashSet},
    convert::TryFrom,
};

fn solve(input: &str) -> u16 {
    let distances: HashMap<_, _> = input
        .lines()
        .map(|l| Distance::try_from(l).unwrap())
        .map(|d| (d.cities, d.distance))
        .collect();

    let cities: HashSet<_> = distances.keys().flat_map(|d| vec![d.a, d.b]).collect();

    let mut queue: BinaryHeap<_> = cities
        .iter()
        .map(|city| {
            let mut remaining = cities.clone();
            remaining.remove(city);
            Path {
                distance: 0,
                city,
                remaining,
            }
        })
        .collect();

    while let Some(path) = queue.pop() {
        if path.remaining.is_empty() {
            return path.distance;
        }

        let new_paths = path.remaining.iter().map(|city| {
            let mut remaining = path.remaining.clone();
            remaining.remove(city);
            let pair = CityPair::new(path.city, city);
            let distance = path.distance + distances[&pair];

            Path {
                remaining,
                city,
                distance,
            }
        });

        queue.extend(new_paths);
    }

    unreachable!();
}

#[derive(Eq, Debug)]
struct Path<'s> {
    remaining: HashSet<&'s str>,
    city: &'s str,
    distance: u16,
}

impl<'s> PartialEq for Path<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<'s> PartialOrd for Path<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'s> Ord for Path<'s> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[derive(Debug)]
struct Distance<'s> {
    cities: CityPair<'s>,
    distance: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CityPair<'s> {
    a: &'s str,
    b: &'s str,
}

impl<'s> CityPair<'s> {
    pub fn new(a: &'s str, b: &'s str) -> Self {
        if a < b {
            CityPair { a, b }
        } else {
            CityPair { a: b, b: a }
        }
    }
}

impl<'s> TryFrom<&'s str> for Distance<'s> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &'s str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<from>[^\s]+) to (?P<to>[^\s]+) = (?P<distance>\d+)").unwrap();
        }

        let captures = RE
            .captures(input)
            .ok_or_else(|| "input did not match expected pattern")?;

        let from = captures.name("from").unwrap().as_str();
        let to = captures.name("to").unwrap().as_str();
        let distance = captures.name("distance").unwrap().as_str().parse()?;
        let cities = CityPair::new(from, to);

        Ok(Distance { cities, distance })
    }
}

#[cfg(test)]
mod nine_a {
    use super::*;

    #[test]
    fn test() {
        let input = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(solve(input), 605);
    }
}

common::read_main!();
