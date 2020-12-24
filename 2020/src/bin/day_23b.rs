fn solve(cups: &str) -> u64 {
    let starting_numbers = cups
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut ring = RingCup::new(starting_numbers);
    (0..10_000_000).for_each(|_| ring.crab_move());

    ring.clockwise_from(1).take(2).map(|c| c as u64).product()
}

#[derive(Debug)]
struct RingCup {
    /// cups[number - 1] = next_number -1
    next_cup_idx: Vec<usize>,
    /// current cup - 1
    selected: usize,
}

impl RingCup {
    const LENGTH: usize = 1_000_000;

    pub fn new(initial: Vec<u32>) -> Self {
        // [1...1_000_000];
        let mut next_cup_idx: Vec<_> = (1..Self::LENGTH + 1).collect();

        let first = if initial.len() > 0 {
            // our cups are 1th index but vectors are 0th indexed
            let starting_numbers: Vec<_> = initial.into_iter().map(|n| (n - 1) as usize).collect();

            // link every cup to its next cup
            starting_numbers.windows(2).for_each(|pair| {
                next_cup_idx[pair[0]] = pair[1];
            });

            // the last starting cup links to the first sorted cup
            next_cup_idx[*starting_numbers.last().unwrap()] = starting_numbers.len();
            starting_numbers[0]
        } else {
            0
        };

        // wrap the ring around
        next_cup_idx[Self::LENGTH - 1] = first;

        RingCup {
            next_cup_idx,
            selected: first,
        }
    }

    pub fn crab_move(&mut self) {
        let picked_up: Vec<_> = self.successors(self.selected).take(3).collect();

        // lowest if all the lower numbers have been picked up
        let selected_is_lowest = self.selected <= 3
            && picked_up.iter().filter(|p| **p < self.selected).count() >= self.selected;

        // selected number is at most either the highest number of the selected number (not inclusive)
        let highest = if selected_is_lowest {
            Self::LENGTH
        } else {
            self.selected
        };

        // find the highest possibly selected number that was not picked up
        let destination = (highest.saturating_sub(4)..highest)
            .filter(|p| !picked_up.contains(p))
            .max()
            .unwrap();

        // before: selected -> picked -> next | destination -> x
        // after:  selected -> next | destination -> picked -> x
        self.next_cup_idx.swap(self.selected, picked_up[2]);
        self.next_cup_idx.swap(picked_up[2], destination);

        self.selected = self.next_cup_idx[self.selected];
    }

    pub fn clockwise_from(&self, cup: u32) -> impl Iterator<Item = u32> + '_ {
        self.successors((cup - 1) as usize).map(|c| (c + 1) as u32)
    }

    fn successors(&self, cup: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(self.next_cup_idx[cup]), move |c| {
            Some(self.next_cup_idx[*c])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"389125467";
        assert_eq!(solve(input), 149245887792);
    }
}

common::read_main!();
