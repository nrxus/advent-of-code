use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

fn solve(input: &str) -> u32 {
    energy_after(input, 1000)
}

fn energy_after(input: &str, iterations: usize) -> u32 {
    let mut moons: Vec<_> = input
        .trim()
        .lines()
        .map(|l| Point::try_from(l).unwrap())
        .map(|p| Moon::new(p))
        .collect();

    (0..iterations).for_each(|_| {
        let velocities: Vec<Vector> = moons
            .iter()
            .map(|m| (moons.iter().map(|o| m.gravity_from(o)).sum()))
            .collect();

        moons
            .iter_mut()
            .zip(velocities.iter())
            .for_each(|(m, v)| m.velocity += *v);

        moons.iter_mut().for_each(Moon::update);
    });

    moons.iter().map(Moon::energy).sum()
}

#[derive(Debug)]
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

    fn energy(&self) -> u32 {
        let pot = (self.point.x.abs() + self.point.y.abs() + self.point.z.abs()) as u32;
        let kin = (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as u32;

        pot * kin
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
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

        assert_eq!(energy_after(input, 10), 179);
    }

    #[test]
    fn medium() {
        let input = r"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        assert_eq!(energy_after(input, 100), 1940);
    }
}

common::read_main!();
