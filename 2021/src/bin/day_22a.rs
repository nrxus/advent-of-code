use std::{ops::RangeInclusive, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

fn solve(input: &str) -> usize {
    let steps: Vec<Step> = input.trim().lines().map(|l| l.parse().unwrap()).collect();

    let mut on_cubes: Vec<CubeRange> = vec![];

    for step in steps {
        if *step.cubes.x.start() < -50
            || *step.cubes.x.end() > 50
            || *step.cubes.y.start() < -50
            || *step.cubes.y.end() > 50
            || *step.cubes.z.start() < -50
            || *step.cubes.z.end() > 50
        {
            continue;
        }

        if step.state {
            let new_cubes =
                on_cubes
                    .iter()
                    .fold(vec![step.cubes.clone()], |new_cubes, existing_cube| {
                        new_cubes
                            .into_iter()
                            .flat_map(|c| c.difference(existing_cube))
                            .collect()
                    });

            on_cubes.extend(new_cubes);
        } else {
            on_cubes = on_cubes
                .into_iter()
                .flat_map(|c| c.difference(&step.cubes))
                .collect();
        }
    }

    on_cubes.into_iter().map(|r| r.len()).sum()
}

#[derive(Debug)]
pub struct Step {
    cubes: CubeRange,
    state: bool,
}

#[derive(Clone, Debug)]
pub struct CubeRange {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl CubeRange {
    pub fn len(&self) -> usize {
        let x = self.x.end() - self.x.start() + 1;
        let y = self.y.end() - self.y.start() + 1;
        let z = self.z.end() - self.z.start() + 1;

        x as usize * y as usize * z as usize
    }

    pub fn difference(self, other: &CubeRange) -> Vec<CubeRange> {
        let x = match difference(&self.x, &other.x) {
            Some(x) => x,
            None => return vec![self],
        };

        let y = match difference(&self.y, &other.y) {
            Some(y) => y,
            None => return vec![self],
        };

        let z = match difference(&self.z, &other.z) {
            Some(z) => z,
            None => return vec![self],
        };

        match (x, y, z) {
            (
                Difference::Truncated {
                    remaining: x,
                    removed: _,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: y2,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => {
                vec![
                    CubeRange {
                        x: self.x.clone(),
                        y: self.y,
                        z,
                    },
                    CubeRange {
                        x: self.x,
                        y,
                        z: z2.clone(),
                    },
                    CubeRange { x, y: y2, z: z2 },
                ]
            }
            (
                Difference::Truncated {
                    remaining: x,
                    removed: x2,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: _,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: z_middle,
                },
            ) => vec![
                CubeRange {
                    x,
                    y: self.y.clone(),
                    z: self.z,
                },
                CubeRange {
                    x: x2.clone(),
                    y: self.y.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x2.clone(),
                    y: self.y,
                    z: z2,
                },
                CubeRange {
                    x: x2,
                    y,
                    z: z_middle,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: _,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: y2,
                },
                Difference::Swallowed,
            ) => vec![
                CubeRange {
                    x: self.x,
                    y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x,
                    y: y2,
                    z: self.z,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: x2,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: y_middle,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: _,
                },
            ) => vec![
                CubeRange {
                    x,
                    y: self.y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2.clone(),
                    y: y2,
                    z: self.z,
                },
                CubeRange {
                    x: x2,
                    y: y_middle,
                    z,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: x2,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: y_middle,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x,
                    y: self.y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2.clone(),
                    y: y2,
                    z: self.z,
                },
                CubeRange {
                    x: x2.clone(),
                    y: y_middle.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x2,
                    y: y_middle,
                    z: z2,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: x2,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: _,
                },
                Difference::Swallowed,
            ) => vec![
                CubeRange {
                    x,
                    y: self.y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: y2,
                    z: self.z,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: _,
                },
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x,
                    y: self.y.clone(),
                    z,
                },
                CubeRange {
                    x,
                    y: self.y,
                    z: z2,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: x2,
                },
                Difference::Swallowed,
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x,
                    y: self.y.clone(),
                    z: self.z,
                },
                CubeRange {
                    x: x2.clone(),
                    y: self.y.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x2,
                    y: self.y,
                    z: z2,
                },
            ],
            (
                Difference::Truncated {
                    remaining: x,
                    removed: _,
                },
                Difference::Swallowed,
                Difference::Swallowed,
            ) => vec![CubeRange {
                x,
                y: self.y,
                z: self.z,
            }],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: x_middle,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: _,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x,
                    y: self.y.clone(),
                    z,
                },
                CubeRange {
                    x: x1,
                    y: self.y.clone(),
                    z: z2.clone(),
                },
                CubeRange {
                    x: x2,
                    y: self.y,
                    z: z2.clone(),
                },
                CubeRange {
                    x: x_middle,
                    y,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: x_middle,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: y2,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: self.x,
                    y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x1,
                    y: y2.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: y2.clone(),
                    z: self.z,
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: y2.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x_middle,
                    y: y2,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: _,
                },
                Difference::Truncated {
                    remaining: y,
                    removed: y2,
                },
                Difference::Swallowed,
            ) => vec![
                CubeRange {
                    x: self.x,
                    y,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x1,
                    y: y2.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: y2,
                    z: self.z,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: _,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: y_middle,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: self.y,
                    z,
                },
                CubeRange {
                    x: self.x.clone(),
                    y: y1,
                    z: z2.clone(),
                },
                CubeRange {
                    x: self.x,
                    y: y2,
                    z: z2.clone(),
                },
                CubeRange {
                    x: x1,
                    y: y_middle.clone(),
                    z: z2.clone(),
                },
                CubeRange {
                    x: x2,
                    y: y_middle,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: x_middle,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: y_middle,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: x1,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: y2,
                    z: self.z,
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: y_middle.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x_middle,
                    y: y_middle,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: x_middle,
                },
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: _,
                },
                Difference::Swallowed,
            ) => vec![
                CubeRange {
                    x: x1,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x_middle,
                    y: y2,
                    z: self.z,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: _,
                },
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x,
                    y: self.y.clone(),
                    z,
                },
                CubeRange {
                    x: x1,
                    y: self.y.clone(),
                    z: z2.clone(),
                },
                CubeRange {
                    x: x2,
                    y: self.y,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: x_middle,
                },
                Difference::Swallowed,
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: x1,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x2,
                    y: self.y.clone(),
                    z: self.z.clone(),
                },
                CubeRange {
                    x: x_middle.clone(),
                    y: self.y.clone(),
                    z: z1,
                },
                CubeRange {
                    x: x_middle,
                    y: self.y,
                    z: z2,
                },
            ],
            (
                Difference::Split {
                    a: x1,
                    b: x2,
                    middle: _,
                },
                Difference::Swallowed,
                Difference::Swallowed,
            ) => {
                vec![
                    CubeRange {
                        x: x1,
                        y: self.y.clone(),
                        z: self.z.clone(),
                    },
                    CubeRange {
                        x: x2,
                        y: self.y,
                        z: self.z,
                    },
                ]
            }
            (
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: y,
                    removed: _,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: self.y,
                    z,
                },
                CubeRange {
                    x: self.x,
                    y,
                    z: z2,
                },
            ],
            (
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: y,
                    removed: y2,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y,
                    z: self.z,
                },
                CubeRange {
                    x: self.x.clone(),
                    y: y2.clone(),
                    z: z1,
                },
                CubeRange {
                    x: self.x,
                    y: y2,
                    z: z2,
                },
            ],
            (
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: y,
                    removed: _,
                },
                Difference::Swallowed,
            ) => vec![CubeRange {
                x: self.x,
                y,
                z: self.z,
            }],
            (
                Difference::Swallowed,
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: _,
                },
                Difference::Truncated {
                    remaining: z,
                    removed: z2,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: self.y,
                    z,
                },
                CubeRange {
                    x: self.x.clone(),
                    y: y1,
                    z: z2.clone(),
                },
                CubeRange {
                    x: self.x,
                    y: y2,
                    z: z2,
                },
            ],
            (
                Difference::Swallowed,
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: y_middle,
                },
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: self.x.clone(),
                    y: y2,
                    z: self.z,
                },
                CubeRange {
                    x: self.x.clone(),
                    y: y_middle.clone(),
                    z: z1,
                },
                CubeRange {
                    x: self.x,
                    y: y_middle,
                    z: z2,
                },
            ],
            (
                Difference::Swallowed,
                Difference::Split {
                    a: y1,
                    b: y2,
                    middle: _,
                },
                Difference::Swallowed,
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: y1,
                    z: self.z.clone(),
                },
                CubeRange {
                    x: self.x,
                    y: y2,
                    z: self.z,
                },
            ],
            (
                Difference::Swallowed,
                Difference::Swallowed,
                Difference::Truncated {
                    remaining: z,
                    removed: _,
                },
            ) => vec![CubeRange {
                x: self.x,
                y: self.y,
                z,
            }],
            (
                Difference::Swallowed,
                Difference::Swallowed,
                Difference::Split {
                    a: z1,
                    b: z2,
                    middle: _,
                },
            ) => vec![
                CubeRange {
                    x: self.x.clone(),
                    y: self.y.clone(),
                    z: z1,
                },
                CubeRange {
                    x: self.x,
                    y: self.y,
                    z: z2,
                },
            ],
            (Difference::Swallowed, Difference::Swallowed, Difference::Swallowed) => vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug)]
enum Difference {
    Truncated {
        remaining: RangeInclusive<i32>,
        removed: RangeInclusive<i32>,
    },
    Split {
        a: RangeInclusive<i32>,
        b: RangeInclusive<i32>,
        middle: RangeInclusive<i32>,
    },
    Swallowed,
}

fn difference(
    original: &RangeInclusive<i32>,
    to_remove: &RangeInclusive<i32>,
) -> Option<Difference> {
    let a = *original.start()..=(to_remove.start() - 1).min(*original.end());
    let b = (to_remove.end() + 1).max(*original.start())..=*original.end();

    if &a == original || &b == original {
        return None;
    }

    let removed = match (a.is_empty(), b.is_empty()) {
        (true, true) => Difference::Swallowed,
        (true, false) => Difference::Truncated {
            removed: *original.start()..=(b.start() - 1),
            remaining: b,
        },
        (false, true) => Difference::Truncated {
            removed: (a.end() + 1)..=*original.end(),
            remaining: a,
        },
        (false, false) => Difference::Split {
            middle: (a.end() + 1)..=(b.start() - 1),
            a,
            b,
        },
    };

    Some(removed)
}

impl FromStr for Step {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<state>on|off) x=(?P<x>.+),y=(?P<y>.+),z=(?P<z>.+)$").unwrap();
            static ref RANGE_RE: Regex = Regex::new(r"(-?\d+)\.\.(-?\d+)").unwrap();
        }

        let caps = RE
            .captures(input)
            .ok_or_else(|| format!("step: '{}' invalid", input))?;

        let state = match caps.name("state").unwrap().as_str() {
            "on" => true,
            "off" => false,
            e => return Err(format!("invalid state: {}", e).into()),
        };

        let parse_range = |range: &str| -> Result<_, Self::Err> {
            let caps = RANGE_RE
                .captures(range)
                .ok_or_else(|| format!("invalid range: {}", input))?;
            let start = caps.get(1).unwrap().as_str().parse().unwrap();
            let end = caps.get(2).unwrap().as_str().parse().unwrap();

            Ok(start..=end)
        };

        let x = parse_range(caps.name("x").unwrap().as_str())?;
        let y = parse_range(caps.name("y").unwrap().as_str())?;
        let z = parse_range(caps.name("z").unwrap().as_str())?;

        Ok(Self {
            state,
            cubes: CubeRange { x, y, z },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let input = r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

        assert_eq!(solve(input), 39);
    }

    #[test]
    // #[ignore]
    fn harder_example() {
        let input = r"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

        assert_eq!(solve(input), 590784);
    }
}

common::read_main!();
