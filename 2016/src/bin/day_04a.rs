use regex::Regex;
use std::collections::HashMap;

fn solve(rooms: &str) -> u32 {
    let regex = Regex::new(r"(?P<name>.*)-(?P<id>\d*)\[(?P<checksum>.*)\]").unwrap();

    rooms
        .trim()
        .lines()
        .filter_map(|room| {
            let captures = regex.captures(room).unwrap();
            let name = captures.name("name").unwrap().as_str();
            let id = captures.name("id").unwrap().as_str();
            let checksum = captures.name("checksum").unwrap().as_str();

            let mut chars: Vec<_> = name
                .split('-')
                .flat_map(|r| r.chars())
                .fold(HashMap::new(), |mut counter, c| {
                    *counter.entry(c).or_insert(0) += 1;
                    counter
                })
                .into_iter()
                .collect();

            chars.sort_by(|a, b| a.1.cmp(&b.1).reverse().then(a.0.cmp(&b.0)));

            let matches = chars
                .into_iter()
                .take(5)
                .map(|(c, _)| c)
                .eq(checksum.chars());

            if matches {
                Some(id.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        assert_eq!(solve(input), 1514);
    }
}

common::read_main!();
