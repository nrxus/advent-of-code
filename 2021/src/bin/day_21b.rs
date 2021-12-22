use std::{collections::HashMap, str::FromStr};

fn solve(input: &str) -> u64 {
    let universes_per_move: HashMap<u8, u8> =
        HashMap::from_iter([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);

    let game = Game::from_str(input).unwrap();

    let mut player_one_won = 0;
    let mut player_two_won = 0;
    let mut universes_per_game = HashMap::from_iter([(game, 1_u64)]);

    while !universes_per_game.is_empty() {
        let length = universes_per_game.len();

        universes_per_game = universes_per_game.into_iter().fold(
            HashMap::with_capacity(length),
            |mut acc, (game, universes)| {
                let player_one = game.player_one;
                let player_two = game.player_two;

                universes_per_move
                    .iter()
                    .filter_map(|(&movement, &new_universes)| {
                        let universes = universes * (new_universes as u64);
                        match player_one.turn(movement) {
                            State::Running(player_one) => Some((
                                Game {
                                    player_one,
                                    player_two,
                                },
                                universes,
                            )),
                            State::Won => {
                                player_one_won += universes;
                                None
                            }
                        }
                    })
                    .for_each(|(game, universes)| {
                        *acc.entry(game).or_default() += universes;
                    });

                acc
            },
        );

        universes_per_game = universes_per_game.into_iter().fold(
            HashMap::with_capacity(length),
            |mut acc, (game, universes)| {
                let player_one = game.player_one;
                let player_two = game.player_two;

                universes_per_move
                    .iter()
                    .filter_map(|(&movement, &new_universes)| {
                        let universes = universes * (new_universes as u64);
                        match player_two.turn(movement) {
                            State::Running(player_two) => Some((
                                Game {
                                    player_one,
                                    player_two,
                                },
                                universes,
                            )),
                            State::Won => {
                                player_two_won += universes;
                                None
                            }
                        }
                    })
                    .for_each(|(game, universes)| {
                        *acc.entry(game).or_default() += universes;
                    });

                acc
            },
        );
    }

    player_one_won.max(player_two_won)
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Game {
    player_one: Player,
    player_two: Player,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Player {
    score: u8,
    position: u8,
}

pub enum State {
    Running(Player),
    Won,
}

impl Player {
    pub fn turn(mut self, move_pawn: u8) -> State {
        self.position = (self.position + move_pawn - 1) % 10 + 1;
        self.score += self.position;

        if self.score >= 21 {
            State::Won
        } else {
            State::Running(self)
        }
    }
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s
            .trim()
            .split_once("\n")
            .ok_or_else(|| "not enough players".to_owned())?;

        let (one, two) = match one.strip_prefix("Player 1 starting position: ") {
            Some(one_pos) => {
                let two_pos = two
                    .strip_prefix("Player 2 starting position: ")
                    .ok_or_else(|| "Player 2 missing".to_owned())?;
                (one_pos, two_pos)
            }
            None => todo!(),
        };

        Ok(Game {
            player_one: Player {
                position: one.parse()?,
                score: 0,
            },
            player_two: Player {
                position: two.parse()?,
                score: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Player 1 starting position: 4
Player 2 starting position: 8";

        assert_eq!(solve(input), 444356092776315);
    }
}

common::read_main!();
