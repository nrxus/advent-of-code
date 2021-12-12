use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let connections: HashMap<Cave, HashSet<Cave>> = input
        .trim()
        .lines()
        .map(|c| Connection::try_from(c).unwrap())
        .fold(HashMap::new(), |mut acc, conn| {
            acc.entry(conn.0).or_default().insert(conn.1);
            acc.entry(conn.1).or_default().insert(conn.0);

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

#[derive(Eq)]
pub struct Path<'s> {
    last: Cave<'s>,
    previous: Vec<Cave<'s>>,
    small_caves: HashSet<Cave<'s>>,
}

impl std::hash::Hash for Path<'_> {
    // deliberately skip small_caves
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.last.hash(state);
        self.previous.hash(state);
    }
}

impl PartialEq for Path<'_> {
    // deliberately skip small_caves
    fn eq(&self, other: &Self) -> bool {
        self.last == other.last && self.previous == other.previous
    }
}

impl<'s> Path<'s> {
    pub fn new() -> Path<'s> {
        Path {
            last: Cave::Start,
            previous: vec![],
            small_caves: HashSet::new(),
        }
    }

    pub fn extend(self, next: impl Iterator<Item = &'s Cave<'s>>) -> Vec<Self> {
        next.filter(|c| match c {
            Cave::Start => false, // never go back to the start
            Cave::End => true, // probably okay but we may want to check that last never equals end
            Cave::Small(c) => !self.small_caves.contains(&Cave::Small(c)),
            Cave::Big(_) => true,
        })
        .map(|&c| {
            let mut previous = self.previous.clone();
            previous.push(self.last);
            let mut small_caves = self.small_caves.clone();
            if let Cave::Small(c) = c {
                small_caves.insert(Cave::Small(c));
            }

            Path {
                last: c,
                previous,
                small_caves,
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
        assert_eq!(solve(input), 10);
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
        assert_eq!(solve(input), 19);
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
        assert_eq!(solve(input), 226);
    }
}

common::read_main!();
