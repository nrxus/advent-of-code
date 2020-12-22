use std::collections::VecDeque;

fn solve(decks: &str) -> u32 {
    let mut decks = decks.trim().split("\n\n").map(|d| {
        d.lines()
            .skip(1)
            .map(|c| c.parse().unwrap())
            .collect::<VecDeque<u32>>()
    });

    let mut player_one = decks.next().unwrap();
    let mut player_two = decks.next().unwrap();

    while let Some(first) = player_one.pop_front() {
        let second = match player_two.pop_front() {
            Some(second) => second,
            None => {
                player_one.push_front(first);
                break;
            }
        };

        if first > second {
            player_one.push_back(first);
            player_one.push_back(second);
        } else {
            player_two.push_back(second);
            player_two.push_back(first);
        }
    }

    player_one
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (c * (i as u32 + 1)))
        .sum::<u32>()
        + player_two
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (c * (i as u32 + 1)))
            .sum::<u32>()
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
        assert_eq!(solve(input), 306);
    }
}

common::read_main!();
