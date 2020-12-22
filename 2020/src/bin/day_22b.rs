use std::collections::{HashSet, VecDeque};

fn solve(decks: &str) -> usize {
    let mut decks = decks.trim().split("\n\n").map(|d| {
        d.lines()
            .skip(1)
            .map(|c| c.parse().unwrap())
            .collect::<VecDeque<usize>>()
    });

    let mut player_one = decks.next().unwrap();
    let mut player_two = decks.next().unwrap();

    let winnning_deck = match game(&mut player_one, &mut player_two) {
        Player::One => player_one,
        Player::Two => player_two,
    };

    winnning_deck
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (c * (i + 1)))
        .sum::<usize>()
}

fn game(player_one: &mut VecDeque<usize>, player_two: &mut VecDeque<usize>) -> Player {
    let mut previous = HashSet::with_capacity(90);
    previous.insert((player_one.clone(), player_two.clone()));

    while let Some(first) = player_one.pop_front() {
        let second = match player_two.pop_front() {
            Some(second) => second,
            None => {
                player_one.push_front(first);
                break;
            }
        };

        let round_winner = if first <= player_one.len() && second <= player_two.len() {
            let mut player_one = player_one.iter().take(first).copied().collect();
            let mut player_two = player_two.iter().take(second).copied().collect();
            game(&mut player_one, &mut player_two)
        } else {
            if first > second {
                Player::One
            } else {
                Player::Two
            }
        };

        match round_winner {
            Player::One => {
                player_one.push_back(first);
                player_one.push_back(second);
            }
            Player::Two => {
                player_two.push_back(second);
                player_two.push_back(first);
            }
        }

        if !previous.insert((player_one.clone(), player_two.clone())) {
            return Player::One;
        }
    }

    if player_one.is_empty() {
        Player::Two
    } else {
        Player::One
    }
}

#[derive(Debug)]
enum Player {
    One,
    Two,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(solve(input), 291);
    }

    #[test]
    fn infinite_if_bug_example() {
        let input = r"Player 1:
43
19

Player 2:
2
29
14";
        assert_eq!(solve(input), 105);
    }
}

common::read_main!();
