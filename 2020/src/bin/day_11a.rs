fn solve(layout: &str) -> usize {
    let mut layout: Layout = layout.parse().unwrap();
    layout.run();
    layout.occupied()
}

#[derive(Debug)]
struct Layout {
    width: usize,
    map: Vec<State>,
}

impl Layout {
    pub fn run(&mut self) {
        loop {
            let changes = self.step();
            if changes.is_empty() {
                break;
            };
            changes.into_iter().for_each(|(i, state)| {
                self.map[i] = state;
            })
        }
    }

    pub fn occupied(&self) -> usize {
        self.map
            .iter()
            .filter(|s| match s {
                State::Occupied => true,
                _ => false,
            })
            .count()
    }

    fn step(&self) -> Vec<(usize, State)> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                State::Floor => None,
                State::Seat => {
                    if self.adjacent(i).iter().any(|x| *x == Some(State::Occupied)) {
                        None
                    } else {
                        Some((i, State::Occupied))
                    }
                }
                State::Occupied => {
                    if self
                        .adjacent(i)
                        .iter()
                        .filter(|x| **x == Some(State::Occupied))
                        .take(4)
                        .count()
                        >= 4
                    {
                        Some((i, State::Seat))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    fn adjacent(&self, i: usize) -> [Option<State>; 8] {
        let mut neighbors = [None, None, None, None, None, None, None, None];
        let has_up = i >= self.width;
        let has_left = i % self.width != 0;
        let has_right = i % self.width != (self.width - 1);
        let has_down = i + self.width < self.map.len();

        if has_up {
            neighbors[0] = Some(self.map[i - self.width]);
        }
        if has_left {
            neighbors[1] = Some(self.map[i - 1]);
        }
        if has_right {
            neighbors[2] = Some(self.map[i + 1]);
        }
        if has_down {
            neighbors[3] = Some(self.map[i + self.width]);
        }
        if has_up && has_left {
            neighbors[4] = Some(self.map[i - self.width - 1]);
        }
        if has_up && has_right {
            neighbors[5] = Some(self.map[i - self.width + 1]);
        }
        if has_down && has_left {
            neighbors[6] = Some(self.map[i + self.width - 1]);
        }
        if has_down && has_right {
            neighbors[7] = Some(self.map[i + self.width + 1]);
        }
        neighbors
    }
}

impl std::str::FromStr for Layout {
    type Err = Error;

    fn from_str(input: &str) -> Result<Layout, Error> {
        let input = input.trim();
        let width = input.lines().next().ok_or(Error)?.len();
        let map = input
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                'L' => Ok(State::Seat),
                '.' => Ok(State::Floor),
                '#' => Ok(State::Occupied),
                _ => Err(Error),
            })
            .collect::<Result<_, _>>()?;
        Ok(Layout { width, map })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Floor,
    Seat,
    Occupied,
}

#[derive(Debug)]
struct Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve(input), 37);
    }
}

common::read_main!();
