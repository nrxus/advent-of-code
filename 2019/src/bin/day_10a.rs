use std::collections::HashSet;

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

    map.iter()
        .map(|satellite| {
            let unique_unit_vectors: HashSet<_> = map
                .iter()
                .filter(|m| *m != satellite)
                .map(|m| satellite.displacement(&m).unit())
                .collect();
            unique_unit_vectors.len()
        })
        .max()
        .unwrap()
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
    fn unit(self) -> Self {
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

impl Point {
    fn displacement(&self, other: &Self) -> Vector {
        Vector {
            x: other.x as isize - self.x as isize,
            y: other.y as isize - self.y as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let input = r".#..#
.....
#####
....#
...##";

        assert_eq!(solve(input), 8);
    }

    #[test]
    fn medium_a() {
        let input = r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        assert_eq!(solve(input), 33);
    }

    #[test]
    fn medium_b() {
        let input = r"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        assert_eq!(solve(input), 35);
    }

    #[test]
    fn medium_c() {
        let input = r".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        assert_eq!(solve(input), 41);
    }

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

        assert_eq!(solve(input), 210);
    }
}

common::read_main!();
