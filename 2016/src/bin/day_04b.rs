use regex::Regex;
use std::collections::HashMap;

fn solve(rooms: &str) -> String {
    let regex = Regex::new(r"(?P<name>.*)-(?P<id>\d*)\[(?P<checksum>.*)\]").unwrap();

    let names: Vec<String> = rooms
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
                Some((name, id.parse::<u32>().unwrap()))
            } else {
                None
            }
        })
        .filter_map(|(name, id)| {
            let length = (b'a'..=b'z').len();
            let name: String = name
                .bytes()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        let new = (((c - b'a') as usize + id as usize) % length) as u8;
                        (b'a' + new) as char
                    } else {
                        ' '
                    }
                })
                .collect();

            if name.contains("northpole") {
                Some(format!("{}: {}", name, id))
            } else {
                None
            }
        })
        .collect();

    names.join("\n")
}

common::read_main!();
