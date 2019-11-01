use common::extensions::*;

pub fn parse(input: &str) -> Result<Vec<(u16, u16)>, std::num::ParseIntError> {
    input
        .trim()
        .lines()
        .map(|l| l.split(','))
        .filter_map(|mut l| l.next().merge(l.next()))
        .map(|(c, r)| (c.trim(), r.trim()))
        .map(|(c, r)| c.parse().merge(r.parse()))
        .collect()
}
