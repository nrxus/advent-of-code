use std::str::FromStr;

fn solve(input: &str) -> u32 {
    let mut game = Game::from_str(input).unwrap();
    let mut dice = (1..=100_u16).cycle().enumerate();

    loop {
        let movement: u16 = dice.by_ref().take(3).map(|(_, roll)| roll).sum();
        if game.player_one.turn(movement) {
            break dice.next().map(|(i, _)| i as u32).unwrap() * game.player_two.score;
        }

        let movement: u16 = dice.by_ref().take(3).map(|(_, roll)| roll).sum();
        if game.player_two.turn(movement) {
            break dice.next().map(|(i, _)| i as u32).unwrap() * game.player_one.score;
        }
    }
}

#[derive(Debug)]
struct Game {
    player_one: Player,
    player_two: Player,
}

#[derive(Debug)]
pub struct Player {
    score: u32,
    position: u8,
}

impl Player {
    pub fn turn(&mut self, move_pawn: u16) -> bool {
        self.position = ((self.position as u16 + move_pawn - 1) % 10 + 1) as u8;
        self.score += self.position as u32;

        self.score >= 1000
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

        assert_eq!(solve(input), 739785);
    }
}

common::read_main!();
