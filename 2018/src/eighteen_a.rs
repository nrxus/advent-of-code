use std::str::FromStr;

fn solve(input: &str) -> usize {
    let mut world: World = input.parse().unwrap();
    (0..10).for_each(|_| {
        world.tick();
    });
    let (trees, lumbers) = world
        .acres
        .into_iter()
        .fold((0, 0), |(lumbers, trees), a| match a {
            Acre::Lumberyard => (lumbers + 1, trees),
            Acre::Trees => (lumbers, trees + 1),
            Acre::Ground => (lumbers, trees),
        });
    trees * lumbers
}

#[derive(Debug)]
struct World {
    acres: Vec<Acre>,
    cols: usize,
}

impl World {
    fn tick(&mut self) {
        self.acres = self
            .acres
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let neighbors = self.neighbors(i);
                a.tick(neighbors.iter().filter_map(|n| *n))
            })
            .collect();
    }

    fn neighbors(&self, index: usize) -> [Option<Acre>; 8] {
        let up_i = index.checked_sub(self.cols);
        let down_i = if index + self.cols < self.acres.len() {
            Some(index + self.cols)
        } else {
            None
        };

        let up = up_i.map(|i| self.acres[i]);
        let down = down_i.map(|i| self.acres[i]);
        let left = if index % self.cols > 0 {
            Some(self.acres[index - 1])
        } else {
            None
        };
        let right = if index % self.cols < self.cols - 1 {
            Some(self.acres[index + 1])
        } else {
            None
        };
        let up_left = up_i.filter(|_| left.is_some()).map(|i| self.acres[i - 1]);
        let up_right = up_i.filter(|_| right.is_some()).map(|i| self.acres[i + 1]);
        let down_left = down_i.filter(|_| left.is_some()).map(|i| self.acres[i - 1]);

        let down_right = down_i
            .filter(|_| right.is_some())
            .map(|i| self.acres[i + 1]);

        [
            up, down, left, right, up_left, up_right, down_left, down_right,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Acre {
    Ground,
    Trees,
    Lumberyard,
}

impl Acre {
    fn tick(self, neighbors: impl Iterator<Item = Acre>) -> Acre {
        match self {
            Acre::Ground => {
                if neighbors.filter(|a| *a == Acre::Trees).count() >= 3 {
                    Acre::Trees
                } else {
                    self
                }
            }

            Acre::Trees => {
                if neighbors.filter(|a| *a == Acre::Lumberyard).count() >= 3 {
                    Acre::Lumberyard
                } else {
                    self
                }
            }
            Acre::Lumberyard => {
                let mut has_trees = false;
                let mut has_lumbers = false;
                for n in neighbors {
                    match n {
                        Acre::Lumberyard => has_lumbers = true,
                        Acre::Trees => has_trees = true,
                        _ => continue,
                    }
                    if has_trees && has_lumbers {
                        break;
                    }
                }
                if !(has_trees && has_lumbers) {
                    Acre::Ground
                } else {
                    self
                }
            }
        }
    }
}

#[derive(Debug)]
enum Never {}

impl FromStr for World {
    type Err = Never;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let cols = lines.next().unwrap().len();
        let acres = input
            .lines()
            .flat_map(str::chars)
            .map(|a| match a {
                '.' => Acre::Ground,
                '#' => Acre::Lumberyard,
                '|' => Acre::Trees,
                _ => panic!("did not expect: '{}'", a),
            })
            .collect();
        Ok(World { acres, cols })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        assert_eq!(solve(input), 1147);
    }
}

common::read_main!();

use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.acres.iter().enumerate().try_for_each(|(i, a)| {
            let c = match a {
                Acre::Ground => '.',
                Acre::Lumberyard => '#',
                Acre::Trees => '|',
            };

            if i % self.cols == self.cols - 1 {
                writeln!(f, "{}", c)
            } else {
                write!(f, "{}", c)
            }
        })
    }
}
