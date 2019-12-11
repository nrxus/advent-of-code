#![feature(drain_filter)]
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let input = input.trim();

    let map: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, point)| {
                if point == '#' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect();

    let satellite = map
        .iter()
        .max_by_key(|&satellite| {
            let unique_vectors: HashSet<_> = map
                .iter()
                .filter(|m| *m != satellite)
                .map(|m| (*m - *satellite).normalize())
                .collect();
            unique_vectors.len()
        })
        .cloned()
        .unwrap();

    let mut meteors: Vec<_> = map.into_iter().filter(|m| *m != satellite).collect();
    meteors.sort_unstable_by(|a, b| {
        (*b - satellite)
            .manhattan()
            .cmp(&(*a - satellite).manhattan())
    });

    let mut ray_collisions: HashMap<Vector, Vec<_>> = HashMap::new();

    for meteor in meteors.into_iter() {
        let distance = (meteor - satellite).normalize();
        ray_collisions.entry(distance).or_default().push(meteor);
    }

    let mut sorted_rays: Vec<_> = ray_collisions.keys().cloned().collect();
    sorted_rays.sort_unstable_by(|a, b| {
        let quadrant = |p: &Vector| {
            if p.x >= 0 && p.y <= 0 {
                1
            } else if p.x >= 0 {
                2
            } else if p.y > 0 {
                3
            } else {
                4
            }
        };
        quadrant(a)
            .cmp(&quadrant(b))
            .then_with(|| (a.y * b.x).cmp(&(b.y * a.x)))
    });

    let mut destroyed_count = 0;
    let mut two_hundredth = None;
    for path in sorted_rays.iter().cycle() {
        if let Some(destroyed) = ray_collisions
            .get_mut(path)
            .and_then(|meteors| meteors.pop())
        {
            destroyed_count += 1;
            if destroyed_count == 200 {
                two_hundredth = Some(destroyed);
                break;
            }
        }
    }

    let two_hundredth = two_hundredth.unwrap();
    two_hundredth.x * 100 + two_hundredth.y
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let res = a % b;
        a = std::mem::replace(&mut b, res);
    }
    a
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn normalize(self) -> Self {
        let gcd = gcd(self.x.abs() as usize, self.y.abs() as usize);
        self / gcd as isize
    }
}

impl std::ops::Div<isize> for Vector {
    type Output = Self;

    fn div(self, rhs: isize) -> Self {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector {
            x: self.x as isize - rhs.x as isize,
            y: self.y as isize - rhs.y as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large() {
        let input = r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(solve(input), 802);
    }
}

common::read_main!();
