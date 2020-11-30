#![feature(try_trait)]

use std::{collections::HashSet, num::ParseIntError, option::NoneError, str::FromStr};

fn solve(input: &str) -> u16 {
    let mut clean_ids = Vec::with_capacity(100);
    let mut fabric = Fabric::new();

    let claims = input.lines().map(|l| l.parse::<Claim>().unwrap());
    for c in claims {
        clean_ids.push(c.id);
        let conflicts = fabric.claim(&c);
        clean_ids.retain(|clean| !conflicts.contains(clean));
    }

    clean_ids[0]
}

struct Fabric {
    grid: Vec<FabricState>,
}

impl Fabric {
    fn new() -> Self {
        Fabric {
            grid: vec![FabricState::Unclaimed; 1000 * 1000],
        }
    }

    fn claim(&mut self, claim: &Claim) -> HashSet<u16> {
        claim
            .iter()
            .flat_map(|p| self.claim_at(p, claim.id))
            .collect()
    }

    fn claim_at(&mut self, (column, row): (usize, usize), id: u16) -> Vec<u16> {
        let index = row * 1000 + column;
        self.grid[index].claim(id)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FabricState {
    Unclaimed,
    Claimed(u16),
    Conflict,
}

impl FabricState {
    fn claim(&mut self, id: u16) -> Vec<u16> {
        use self::FabricState::*;

        match *self {
            Unclaimed => {
                *self = Claimed(id);
                vec![]
            }
            Claimed(old_id) => {
                *self = Conflict;
                vec![old_id, id]
            }
            Conflict => {
                *self = Conflict;
                vec![id]
            }
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: u16,
    bottom_right: (usize, usize),
    top_left: (usize, usize),
}

impl Claim {
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        ClaimIter {
            claim: self,
            pointer: self.top_left,
        }
    }
}

struct ClaimIter<'c> {
    claim: &'c Claim,
    pointer: (usize, usize),
}

impl Iterator for ClaimIter<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer.0 == self.claim.bottom_right.0 {
            self.pointer.0 = self.claim.top_left.0;
            self.pointer.1 += 1;
        }

        if self.pointer.1 == self.claim.bottom_right.1 {
            None
        } else {
            let pointer = self.pointer;
            self.pointer.0 += 1;
            Some(pointer)
        }
    }
}

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split('@');
        let id = line.next()?.trim()[1..].parse()?;
        let mut line = line.next()?.split(':');
        let mut top_left = line.next()?.trim().split(',');
        let left = top_left.next()?.parse()?;
        let top = top_left.next()?.parse()?;
        let mut size = line.next()?.trim().split('x');
        let right = left + size.next()?.parse::<usize>()?;
        let bottom = top + size.next()?.parse::<usize>()?;
        Ok(Claim {
            id,
            top_left: (left, top),
            bottom_right: (right, bottom),
        })
    }
}

#[derive(Debug)]
struct ClaimParseError;

impl From<NoneError> for ClaimParseError {
    fn from(_: NoneError) -> Self {
        ClaimParseError
    }
}

impl From<ParseIntError> for ClaimParseError {
    fn from(_: ParseIntError) -> Self {
        ClaimParseError
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;
        assert_eq!(solve(input), 3);
    }
}

common::bootstrap!(3);
