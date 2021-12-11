use std::{
    collections::HashSet,
    fmt::{self, Formatter},
    str::FromStr,
};

fn solve(input: &str) -> usize {
    const STEPS: u16 = 100;
    let mut cavern = Cavern::from_str(input).unwrap();

    (0..STEPS).map(|_| cavern.run_step()).sum()
}

#[derive(Debug)]
struct Cavern {
    grid: [u8; 100],
}

impl Cavern {
    const LEN: usize = 10;

    pub fn run_step(&mut self) -> usize {
        self.grid.iter_mut().for_each(|v| *v = (*v + 1) % 10);

        let mut frontier: Vec<usize> = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(i, v)| Some(i).filter(|_| *v == 0))
            .collect();

        let mut flashed: HashSet<usize> = frontier.iter().copied().collect();

        while let Some(i) = frontier.pop() {
            let flasing_neighbors: Vec<_> = self
                .neighbor_indices(i)
                .into_iter()
                .flatten()
                .filter(|&i| {
                    if flashed.contains(&i) {
                        return false;
                    }

                    self.grid[i] = (self.grid[i] + 1) % 10;
                    self.grid[i] == 0
                })
                .collect();

            flashed.extend(flasing_neighbors.iter().copied());
            frontier.extend(flasing_neighbors);
        }

        flashed.len()
    }

    fn neighbor_indices(&self, index: usize) -> [Option<usize>; 8] {
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
    }
}

impl std::str::FromStr for Cavern {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [0; Self::LEN * Self::LEN];
        s.trim()
            .lines()
            .flat_map(|s| s.chars())
            .enumerate()
            .try_for_each(|(i, g)| match g.to_digit(10) {
                Some(d) => {
                    grid[i] = d as u8;
                    Ok(())
                }
                None => Err("input character is not a digit".to_string()),
            })?;

        Ok(Self { grid })
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.grid
            .chunks_exact(Self::LEN)
            .try_for_each(|line| writeln!(f, "{:?}", line))
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
        assert_eq!(solve(input), 1656);
    }
}

common::read_main!();
