use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

fn solve(input: &str) -> usize {
    let moons: Vec<_> = input
        .trim()
        .lines()
        .map(|l| Point::try_from(l).unwrap())
        .map(|p| Moon::new(p))
        .collect();

    let mut space = Space { moons };
    let (ox, oy, oz) = space.split_coords();

    let (mut x_loop, mut y_loop, mut z_loop) = (None, None, None);

    let (x_loop, y_loop, z_loop) = (1..)
        .find_map(|i| {
            space.update();

            let (x, y, z) = space.split_coords();
            if x == ox && x_loop == None {
                x_loop = Some(i);
            }
            if y == oy && y_loop == None {
                y_loop = Some(i);
            }
            if z == oz && z_loop == None {
                z_loop = Some(i);
            }

            if let Some(_) = x_loop.and(y_loop).and(z_loop) {
                Some((x_loop.unwrap(), y_loop.unwrap(), z_loop.unwrap()))
            } else {
                None
            }
        })
        .unwrap();

    let xy_loop = (x_loop * y_loop) / gcd(x_loop, y_loop);
    let xyz_loop = (xy_loop * z_loop) / gcd(xy_loop, z_loop);

    xyz_loop
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let res = a % b;
        a = std::mem::replace(&mut b, res);
    }
    a
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Space {
    moons: Vec<Moon>,
}

impl Space {
    fn update(&mut self) {
        let velocities: Vec<Vector> = self
            .moons
            .iter()
            .map(|m| (self.moons.iter().map(|o| m.gravity_from(o)).sum()))
            .collect();

        self.moons
            .iter_mut()
            .zip(velocities.iter())
            .for_each(|(m, v)| m.velocity += *v);

        self.moons.iter_mut().for_each(Moon::update);
    }

    fn split_coords(&self) -> (Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let x = self
            .moons
            .iter()
            .map(|m| (m.point.x, m.velocity.x))
            .collect();
        let y = self
            .moons
            .iter()
            .map(|m| (m.point.y, m.velocity.y))
            .collect();
        let z = self
            .moons
            .iter()
            .map(|m| (m.point.z, m.velocity.z))
            .collect();

        (x, y, z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Moon {
    point: Point,
    velocity: Vector,
}

impl Moon {
    fn new(point: Point) -> Self {
        Moon {
            point,
            velocity: Vector::new(),
        }
    }

    fn gravity_from(&self, other: &Moon) -> Vector {
        Vector {
            x: (other.point.x.cmp(&self.point.x)) as i32,
            y: (other.point.y.cmp(&self.point.y)) as i32,
            z: (other.point.z.cmp(&self.point.z)) as i32,
        }
    }

    fn update(&mut self) {
        self.point += self.velocity;
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl std::iter::Sum for Vector {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Self {
        iter.fold(Vector::new(), |acc, v| acc + v)
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Vector {
    fn new() -> Vector {
        Vector::default()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(input: &str) -> Result<Point, String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
        }
        let captures = RE
            .captures(input)
            .ok_or_else(|| "input did not match pattern")?;
        Ok(Point {
            x: captures.name("x").unwrap().as_str().parse().unwrap(),
            y: captures.name("y").unwrap().as_str().parse().unwrap(),
            z: captures.name("z").unwrap().as_str().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let input = r"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        assert_eq!(solve(input), 2772);
    }

    #[test]
    fn medium() {
        let input = r"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        assert_eq!(solve(input), 4686774924);
    }
}

common::read_main!();
