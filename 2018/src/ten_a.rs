use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Clone)]
struct Message {
    points: Vec<Point>,
}

impl Message {
    fn advance(&mut self) {
        self.points.iter_mut().for_each(|p| {
            p.position.0 += p.velocity.0;
            p.position.1 += p.velocity.1;
        })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.points.iter().map(|p| p.position.0).min().unwrap();
        let max_x = self.points.iter().map(|p| p.position.0).max().unwrap();
        let min_y = self.points.iter().map(|p| p.position.1).min().unwrap();
        let max_y = self.points.iter().map(|p| p.position.1).max().unwrap();

        (min_y..=max_y).try_for_each(|y| {
            (min_x..=max_x).try_for_each(|x| {
                let c = if self.points.iter().any(|p| p.position == (x, y)) {
                    "#"
                } else {
                    "."
                };
                f.write_str(c)
            })?;
            f.write_str("\n")
        })
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        //position=<-20620, -41485> velocity=< 2,  4>
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"position=<\s*(?P<p_x>-?\d+),\s*(?P<p_y>-?\d+)> velocity=<\s*(?P<v_x>-?\d+),\s*(?P<v_y>-?\d+)>"
            )
            .unwrap();
        }

        let caps = RE.captures(input).unwrap();
        let p_x: i32 = caps.name("p_x").unwrap().as_str().parse()?;
        let p_y: i32 = caps.name("p_y").unwrap().as_str().parse()?;
        let v_x: i32 = caps.name("v_x").unwrap().as_str().parse()?;
        let v_y: i32 = caps.name("v_y").unwrap().as_str().parse()?;

        Ok(Point {
            position: (p_x, p_y),
            velocity: (v_x, v_y),
        })
    }
}

fn solve(input: &str) -> String {
    let mut message = input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map(|points| Message { points })
        .unwrap();

    (0..)
        .scan(999_999, |width, _| {
            message.advance();
            let min_x = message.points.iter().map(|p| p.position.0).min().unwrap();
            let max_x = message.points.iter().map(|p| p.position.0).max().unwrap();
            let new_width = max_x - min_x;
            if *width < new_width {
                None
            } else {
                *width = new_width;
                Some(message.clone())
            }
        })
        .last()
        .unwrap()
        .to_string()
}

common::read_main!();
