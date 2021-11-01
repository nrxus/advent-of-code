use array_macro::array;

fn solve(input: &str) -> Answer {
    let input: usize = input.trim().parse().unwrap();
    let mut scores = Vec::<u8>::with_capacity(11 + input);
    scores.extend(&[3, 7]);
    let mut first = 0;
    let mut second = 1;

    while scores.len() < 10 + input {
        let new = scores[first] + scores[second];
        if new > 9 {
            scores.extend(&[1, new % 10]);
        } else {
            scores.push(new);
        }
        first = (first + 1 + scores[first] as usize) % scores.len();
        second = (second + 1 + scores[second] as usize) % scores.len();
    }

    let scores: Vec<_> = scores.into_iter().skip(input).take(10).collect();
    Answer(array![x => scores[x]; 10])
}

#[derive(Default)]
struct Answer([u8; 10]);

use std::fmt;

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|i| write!(f, "{}", i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(&format!("{}", solve("9")), "5158916779");
        assert_eq!(&format!("{}", solve("5")), "0124515891");
        assert_eq!(&format!("{}", solve("18")), "9251071085");
        assert_eq!(&format!("{}", solve("2018")), "5941429882");
    }
}

common::read_main!();
