use std::{collections::HashMap, str::FromStr};

fn solve(input: &str) -> usize {
    let seen: HashMap<Point, usize> = input
        .trim()
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .flat_map(|line| line.points())
        .fold(HashMap::new(), |mut seen, point| {
            *seen.entry(point).or_default() += 1;
            seen
        });

    seen.into_values().filter(|&count| count > 1).count()
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    x: u16,
    y: u16,
}

pub struct Line(Point, Point);

impl Line {
    pub fn points(&self) -> Vec<Point> {
        if self.0.x == self.1.x {
            let min = self.0.y.min(self.1.y);
            let max = self.0.y.max(self.1.y);

            (min..=max).map(|y| Point { x: self.0.x, y }).collect()
        } else if self.0.y == self.1.y {
            let min = self.0.x.min(self.1.x);
            let max = self.0.x.max(self.1.x);

            (min..=max).map(|x| Point { x, y: self.0.y }).collect()
        } else {
            let (left, right) = if self.0.x < self.1.x {
                (&self.0, &self.1)
            } else {
                (&self.1, &self.0)
            };

            let xs = (left.x..=right.x).enumerate();
            if left.y < right.y {
                xs.map(|(i, x)| Point {
                    x,
                    y: left.y + i as u16,
                })
                .collect()
            } else {
                xs.map(|(i, x)| Point {
                    x,
                    y: left.y - i as u16,
                })
                .collect()
            }
        }
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once("->")
            .ok_or_else(|| "did not find -> in Line".to_owned())?;
        let a: Point = a.trim().parse()?;
        let b: Point = b.trim().parse()?;

        Ok(Line(a, b))
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .ok_or_else(|| "missing ',' in Point".to_owned())?;

        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(solve(input), 12);
    }
}

common::read_main!();
