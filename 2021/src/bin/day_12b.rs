use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

fn solve(input: &str) -> usize {
    let connections: HashMap<Cave, HashSet<Cave>> = input
        .trim()
        .lines()
        .map(|c| Connection::try_from(c).unwrap())
        .fold(HashMap::new(), |mut acc, conn| {
            if conn.1 != Cave::Start && conn.0 != Cave::End {
                acc.entry(conn.0).or_default().insert(conn.1);
            }
            if conn.0 != Cave::Start && conn.1 != Cave::End {
                acc.entry(conn.1).or_default().insert(conn.0);
            }

            acc
        });

    let mut frontier = vec![Path::new()];
    let mut full_paths: HashSet<Path> = HashSet::new();

    while let Some(path) = frontier.pop() {
        let next = match connections.get(&path.last) {
            None => continue,
            Some(n) => n,
        };
        let next = path.extend(next.iter());
        for path in next {
            if path.last == Cave::End {
                full_paths.insert(path);
            } else {
                frontier.push(path);
            }
        }
    }

    full_paths.len()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path<'s> {
    last: Cave<'s>,
    previous: Vec<Cave<'s>>,
    small_caves: HashSet<Cave<'s>>,
    single_repeat: Option<Cave<'s>>,
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.previous
            .iter()
            .map(|p| match p {
                Cave::Start => "start",
                Cave::End => "end",
                Cave::Small(s) => s,
                Cave::Big(b) => b,
            })
            .try_for_each(|p| {
                f.write_str(p)?;
                f.write_char(',')?;
                Ok(())
            })?;

        f.write_str("end")
    }
}

impl std::hash::Hash for Path<'_> {
    // deliberately ignore small_caves
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.last.hash(state);
        self.previous.hash(state);
    }
}

impl<'s> Path<'s> {
    pub fn new() -> Path<'s> {
        Path {
            last: Cave::Start,
            previous: vec![],
            small_caves: HashSet::new(),
            single_repeat: None,
        }
    }

    pub fn extend(self, next: impl Iterator<Item = &'s Cave<'s>>) -> Vec<Self> {
        next.filter(|c| match c {
            // probably okay but we may want to check that last never equals end
            Cave::End | Cave::Big(_) => true,
            Cave::Start => unreachable!(),
            Cave::Small(c) => {
                self.single_repeat.is_none() || !self.small_caves.contains(&Cave::Small(c))
            }
        })
        .map(|&c| {
            let mut previous = self.previous.clone();
            previous.push(self.last);
            let mut small_caves = self.small_caves.clone();
            let mut single_repeat = self.single_repeat;

            if let Cave::Small(c) = c {
                if !small_caves.insert(Cave::Small(c)) {
                    single_repeat = Some(Cave::Small(c));
                }
            }

            Path {
                last: c,
                previous,
                small_caves,
                single_repeat,
            }
        })
        .collect()
    }
}

impl<'s> Default for Path<'s> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Cave<'s> {
    Start,
    End,
    Small(&'s str),
    Big(&'s str),
}

impl<'s> TryFrom<&'s str> for Cave<'s> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            v if v.chars().all(|v| v.is_lowercase()) => Ok(Self::Small(v)),
            v if v.chars().all(|v| v.is_uppercase()) => Ok(Self::Big(v)),
            _ => Err("invalid cavern name".to_owned().into()),
        }
    }
}

#[derive(Debug)]
struct Connection<'s>(Cave<'s>, Cave<'s>);

impl<'s> TryFrom<&'s str> for Connection<'s> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        let (a, b) = value
            .split_once('-')
            .ok_or_else(|| "missing '-' in connection".to_owned())?;

        Ok(Connection(a.try_into()?, b.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(solve(input), 36);
    }

    #[test]
    fn example_two() {
        let input = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(solve(input), 103);
    }

    #[test]
    fn example_three() {
        let input = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(solve(input), 3509);
    }
}

common::read_main!();
