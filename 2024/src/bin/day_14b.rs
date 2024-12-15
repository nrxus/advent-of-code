use std::{
    collections::HashMap,
    fmt::{self, Formatter, Write},
};

fn solve(input: &str) -> u32 {
    let robots = input
        .trim()
        .lines()
        .map(|robot| {
            let (p, v) = robot.split_once(' ').unwrap();
            let p = p.strip_prefix("p=").unwrap();
            let (x, y) = p.split_once(',').unwrap();
            let p = (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap());

            let v = v.strip_prefix("v=").unwrap();
            let (x, y) = v.split_once(',').unwrap();
            let v = (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap());

            (p, v)
        })
        .fold(HashMap::new(), |mut robots, (p, v)| {
            robots.entry(p).or_insert(vec![]).push(v);
            robots
        });

    let mut map = Map { robots };
    let second = (1..).find(|_| {
        map.next();
        if map.potential_tree() {
            println!("{map}");
            true
        } else {
            false
        }
    });

    second.unwrap()
}

struct Map {
    robots: HashMap<(u16, u16), Vec<(i16, i16)>>,
}

impl Map {
    pub fn next(&mut self) {
        let robots = std::mem::take(&mut self.robots);
        let robots = robots
            .into_iter()
            .flat_map(|(p, vs)| {
                vs.into_iter().map(move |v| {
                    let x = (p.0 as i16 + v.0).rem_euclid(101) as u16;
                    let y = (p.1 as i16 + v.1).rem_euclid(103) as u16;
                    ((x, y), v)
                })
            })
            .fold(HashMap::new(), |mut robots, (p, v)| {
                robots.entry(p).or_insert(vec![]).push(v);
                robots
            });
        self.robots = robots;
    }

    pub fn potential_tree(&self) -> bool {
        self.robots
            .keys()
            .filter(|&&(x, y)| {
                let Some(left) = x.checked_sub(1) else {
                    return false;
                };
                let Some(up) = y.checked_sub(1) else {
                    return false;
                };
                let right = x + 1;
                let down = y + 1;

                [
                    (left, up),
                    (x, up),
                    (right, up),
                    (left, y),
                    (right, y),
                    (left, down),
                    (x, down),
                    (right, down),
                ]
                .into_iter()
                .all(|pos| self.robots.contains_key(&pos))
            })
            .take(2)
            .count()
            == 2
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..103 {
            for x in 0..101 {
                if self.robots.contains_key(&(x, y)) {
                    f.write_char('X')?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

common::read_main!();
