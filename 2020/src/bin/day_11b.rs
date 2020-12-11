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
                    if self.visible(i).iter().any(|x| *x == Some(State::Occupied)) {
                        None
                    } else {
                        Some((i, State::Occupied))
                    }
                }
                State::Occupied => {
                    if self
                        .visible(i)
                        .iter()
                        .filter(|x| **x == Some(State::Occupied))
                        .take(5)
                        .count()
                        >= 5
                    {
                        Some((i, State::Seat))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    fn visible(&self, i: usize) -> [Option<State>; 8] {
        let mut neighbors = [None, None, None, None, None, None, None, None];
        let has_up = |i| i >= self.width;
        let has_left = |i| i % self.width != 0;
        let has_right = |i| i % self.width != (self.width - 1);
        let has_down = |i| i + self.width < self.map.len();
        let go_up = |i| i - self.width;
        let go_left = |i| i - 1;
        let go_right = |i| i + 1;
        let go_down = |i| i + self.width;

        neighbors[0] = self.first_visible(i, has_up, go_up);
        neighbors[1] = self.first_visible(i, has_left, go_left);
        neighbors[2] = self.first_visible(i, has_right, go_right);
        neighbors[3] = self.first_visible(i, has_down, go_down);

        neighbors[4] = self.first_visible(i, |i| has_up(i) && has_left(i), |i| go_left(go_up(i)));
        neighbors[5] = self.first_visible(i, |i| has_up(i) && has_right(i), |i| go_right(go_up(i)));
        neighbors[6] =
            self.first_visible(i, |i| has_down(i) && has_left(i), |i| go_left(go_down(i)));
        neighbors[7] =
            self.first_visible(i, |i| has_down(i) && has_right(i), |i| go_right(go_down(i)));

        neighbors
    }

    fn first_visible(
        &self,
        mut i: usize,
        condition: impl Fn(usize) -> bool,
        change: impl Fn(usize) -> usize,
    ) -> Option<State> {
        while condition(i) {
            i = change(i);
            match self.map[i] {
                State::Floor => continue,
                s => return Some(s),
            }
        }

        None
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
        assert_eq!(solve(input), 26);
    }
}

common::read_main!();
