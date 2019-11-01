#[derive(Default, Debug)]
struct Marble {
    prev: usize,
    next: usize,
    value: u32,
}

#[derive(Debug)]
struct Board {
    marbles: Vec<Marble>,
    current: usize,
}

impl Board {
    pub fn new(capacity: usize) -> Self {
        let mut marbles = Vec::with_capacity(capacity);
        marbles.push(Marble::default());
        Board {
            marbles,
            current: 0,
        }
    }

    pub fn skip_clock(&mut self, skipped: usize) -> &mut Board {
        self.current = (0..skipped).fold(self.current, |current, _| {
            unsafe { self.marbles.get_unchecked(current) }.next
        });
        self
    }

    pub fn skip_counterclock(&mut self, skipped: usize) -> &mut Board {
        self.current = (0..skipped).fold(self.current, |current, _| {
            unsafe { self.marbles.get_unchecked(current) }.prev
        });
        self
    }

    pub fn insert(&mut self, value: u32) {
        let current = self.marbles.len();
        let prev_marble = unsafe { self.marbles.get_unchecked_mut(self.current) };
        let new_marble = Marble {
            prev: self.current,
            next: prev_marble.next,
            value,
        };
        prev_marble.next = current;
        let next_marble = unsafe { self.marbles.get_unchecked_mut(new_marble.next) };
        next_marble.prev = current;

        self.marbles.push(new_marble);
        self.current = current;
    }

    pub fn remove(&mut self) -> u32 {
        let removed = self.marbles.swap_remove(self.current);

        //fix swap
        if self.current < self.marbles.len() {
            unsafe {
                let swapped = self.marbles.get_unchecked(self.current);
                let prev = swapped.prev;
                let next = swapped.next;
                self.marbles.get_unchecked_mut(prev).next = self.current;
                self.marbles.get_unchecked_mut(next).prev = self.current;
            }
        }

        self.current = removed.next;
        unsafe {
            self.marbles.get_unchecked_mut(removed.prev).next = removed.next;
            self.marbles.get_unchecked_mut(removed.next).prev = removed.prev;
        }

        removed.value
    }
}

fn solve(input: &str) -> u32 {
    let mut input = input.split_whitespace();
    let players: usize = input.next().unwrap().parse().unwrap();
    let mut players = vec![0; players];
    let marbles = input.rev().nth(1).unwrap().parse::<u32>().unwrap();
    let rounds = marbles as usize / 23;
    let mut board = Board::new(21 * rounds + 1);
    let mut player = 0;
    (0..rounds).for_each(|round| {
        let last = (round + 1) * 23;
        (round * 23 + 1..last).for_each(|m| {
            board.skip_clock(1).insert(m as u32);
        });
        let removed = board.skip_counterclock(7).remove();
        player = (player + 22) % players.len();
        unsafe {
            *(players.get_unchecked_mut(player)) += last as u32 + removed;
        }
        player += 1;
    });
    players.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"9 players; last marble is worth 25 points";
        assert_eq!(solve(input), 32);
    }

    #[test]
    fn test_b() {
        let input = r"10 players; last marble is worth 1618 points";
        assert_eq!(solve(input), 8317);
    }

    #[test]
    fn test_c() {
        let input = r"13 players; last marble is worth 7999 points";
        assert_eq!(solve(input), 146373);
    }

    #[test]
    fn test_d() {
        let input = r"17 players; last marble is worth 1104 points";
        assert_eq!(solve(input), 2764);
    }

    #[test]
    fn test_e() {
        let input = r"21 players; last marble is worth 6111 points";
        assert_eq!(solve(input), 54718);
    }

    #[test]
    fn test_f() {
        let input = r"30 players; last marble is worth 5807 points";
        assert_eq!(solve(input), 37305);
    }
}

common::read_main!();
//common::bootstrap!(9);
