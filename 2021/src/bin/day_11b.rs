use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Formatter, Write},
    str::FromStr,
};

fn solve(input: &str) -> usize {
    let mut cavern = Cavern::from_str(input).unwrap();

    (1..).find(|_| cavern.run_step() == 100).unwrap()
}

#[derive(Debug)]
struct Cavern {
    grid: [Octopus; 100],
}

impl Cavern {
    const LEN: usize = 10;

    pub fn run_step(&mut self) -> usize {
        let mut frontier: Vec<usize> = self
            .grid
            .iter_mut()
            .enumerate()
            .filter_map(|(i, v)| if v.power_up() { Some(i) } else { None })
            .collect();

        let mut flashed: HashSet<usize> = frontier.iter().copied().collect();

        while let Some(i) = frontier.pop() {
            let flashing_neighbors: Vec<_> = self
                .neighbor_indices(i)
                .into_iter()
                .filter(|&i| {
                    if flashed.contains(&i) {
                        return false;
                    }

                    self.grid[i].power_up()
                })
                .collect();

            flashed.extend(flashing_neighbors.iter().copied());
            frontier.extend(flashing_neighbors);
        }

        flashed.len()
    }

    fn neighbor_indices(&self, index: usize) -> impl Iterator<Item = usize> {
        let left = if index % Self::LEN == 0 {
            None
        } else {
            Some(index - 1)
        };
        let right = Some(index + 1).filter(|&right| right % Self::LEN != 0);
        let up = index.checked_sub(Self::LEN);
        let down = Some(index + Self::LEN).filter(|&down| down < self.grid.len());
        let up_left = up.zip(left).map(|(up, _)| up - 1);
        let up_right = up.zip(right).map(|(up, _)| up + 1);
        let down_left = down.zip(left).map(|(down, _)| down - 1);
        let down_right = down.zip(right).map(|(down, _)| down + 1);

        [
            left, right, up, down, up_left, up_right, down_left, down_right,
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone, Copy)]
struct Octopus(u8);

impl Octopus {
    const MAX_POWER: u8 = 10;

    pub fn new(c: char) -> Result<Self, Box<dyn Error>> {
        c.to_digit(10)
            .map(|d| Self(d as u8))
            .ok_or_else(|| "octopus has to be a digit".to_owned().into())
    }

    pub fn power_up(&mut self) -> bool {
        self.0 = (self.0 + 1) % Self::MAX_POWER;
        self.0 == 0
    }
}

impl std::str::FromStr for Cavern {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [Octopus(0); Self::LEN * Self::LEN];
        s.trim()
            .lines()
            .flat_map(|s| s.chars())
            .enumerate()
            .try_for_each(|(i, g)| match Octopus::new(g) {
                Ok(o) => {
                    grid[i] = o;
                    Ok(())
                }
                Err(e) => Err(e),
            })?;

        Ok(Self { grid })
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.grid.chunks_exact(Self::LEN).try_for_each(|line| {
            line.iter().try_for_each(|o| write!(f, "{}", o.0))?;
            f.write_str("\n")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(solve(input), 195);
    }
}

common::read_main!();
