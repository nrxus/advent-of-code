#![feature(drain_filter)]

use std::str::FromStr;

fn solve(input: &str) -> usize {
    let mut constellations: Vec<Vec<Star>> = vec![];

    input.lines().for_each(|l| {
        let star = l.parse().unwrap();
        let connected = constellations
            .drain_filter(|c| c.iter().any(|s| s.connects(&star)))
            .flatten()
            .chain(std::iter::once(star))
            .collect();
        constellations.push(connected)
    });

    constellations.len()
}

#[derive(Clone, Copy)]
struct Star {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Star {
    fn connects(&self, other: &Self) -> bool {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
            <= 3
    }
}

impl FromStr for Star {
    type Err = std::boxed::Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: Vec<_> = input
            .split(',')
            .map(|c| i32::from_str(c))
            .collect::<Result<_, _>>()?;
        if input.len() != 4 {
            Err("too many numbers in star line".into())
        } else {
            Ok(Star {
                x: input[0],
                y: input[1],
                z: input[2],
                w: input[3],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_b() {
        let input = r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_c() {
        let input = r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_d() {
        let input = r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(solve(input), 8);
    }
}

common::read_main!();
