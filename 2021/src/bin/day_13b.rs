use std::{
    collections::HashSet,
    fmt::{self, Formatter, Write},
    str::FromStr,
};

fn solve(input: &str) -> String {
    let (points, folds) = input.trim().split_once("\n\n").unwrap();
    let paper: Paper = points.parse().unwrap();
    let paper = folds
        .lines()
        .map(|fold| fold.parse().unwrap())
        .fold(paper, |paper, fold| paper.fold(fold));

    paper.to_string()
}

struct Paper {
    points: HashSet<(u16, u16)>,
}

impl Paper {
    pub fn fold(self, fold: Fold) -> Paper {
        let points = self.points.into_iter();

        let points = match fold {
            Fold::Vertical(magnitude) => points
                .map(|(x, y)| {
                    let x = match x.cmp(&magnitude) {
                        std::cmp::Ordering::Less => x,
                        std::cmp::Ordering::Equal => panic!("folding on line with point"),
                        std::cmp::Ordering::Greater => 2 * magnitude - x,
                    };

                    (x, y)
                })
                .collect(),
            Fold::Horizontal(magnitude) => points
                .map(|(x, y)| {
                    let y = match y.cmp(&magnitude) {
                        std::cmp::Ordering::Less => y,
                        std::cmp::Ordering::Equal => panic!("folding on line with point"),
                        std::cmp::Ordering::Greater => 2 * magnitude - y,
                    };

                    (x, y)
                })
                .collect(),
        };

        Paper { points }
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.points.is_empty() {
            return Ok(());
        }

        let max_x = self.points.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = self.points.iter().map(|(_, y)| *y).max().unwrap();

        (0..=max_y).try_for_each(|y| {
            (0..=max_x).try_for_each(|x| {
                if self.points.contains(&(x, y)) {
                    f.write_char('#')
                } else {
                    f.write_char('.')
                }
            })?;

            if y != max_y {
                f.write_str("\n")
            } else {
                Ok(())
            }
        })
    }
}

enum Fold {
    Vertical(u16),
    Horizontal(u16),
}

impl FromStr for Paper {
    type Err = Box<dyn std::error::Error>;

    fn from_str(points: &str) -> Result<Self, Self::Err> {
        let points = points
            .lines()
            .map(|l| {
                let (x, y) = l
                    .split_once(',')
                    .ok_or_else(|| "missing ',' in point".to_owned())?;
                let x: u16 = x.parse()?;
                let y: u16 = y.parse()?;

                Ok((x, y))
            })
            .collect::<Result<_, Self::Err>>()?;

        Ok(Self { points })
    }
}

impl FromStr for Fold {
    type Err = Box<dyn std::error::Error>;

    fn from_str(fold: &str) -> Result<Self, Self::Err> {
        let fold = fold
            .strip_prefix("fold along ")
            .ok_or_else(|| "missing fold prefix".to_owned())?;

        let (direction, magnitude) = fold
            .split_once('=')
            .ok_or_else(|| "missing '=' in fold".to_owned())?;

        let magnitude: u16 = magnitude.parse()?;

        match direction {
            "x" => Ok(Fold::Vertical(magnitude)),
            "y" => Ok(Fold::Horizontal(magnitude)),
            e => Err(format!("invalid direction: {}", e).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7";
        assert_eq!(
            solve(input),
            "#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###"
        );
    }

    #[test]
    fn example_two() {
        let input = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(
            solve(input),
            "#####
#...#
#...#
#...#
#####"
        );
    }
}

common::read_main!();
