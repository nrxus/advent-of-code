use std::{collections::HashMap, ops::AddAssign, str::FromStr};

fn solve(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .trim()
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand: Hand = hand.parse().unwrap();
            let bid: u32 = bid.parse().unwrap();
            (hand, bid)
        })
        .collect();

    hands.sort_by_key(|(h, _)| *h);

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, b))| b * (i as u32 + 1))
        .sum()
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeKind([Card; 5]),
    FullHouse([Card; 5]),
    FourKind([Card; 5]),
    FiveKind([Card; 5]),
}

impl FromStr for Hand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = [Card::A; 5];
        s.chars()
            .map(|c| match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::Joker,
                'T' => Card::T,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => unreachable!(),
            })
            .enumerate()
            .for_each(|(i, card)| hand[i] = card);

        let mut uniques = HashMap::new();
        hand.into_iter()
            .for_each(|c| uniques.entry(c).or_insert(0_usize).add_assign(1));
        let num_jokers = uniques.remove(&Card::Joker).unwrap_or(0);

        let hand = match uniques.len() {
            5 => Hand::HighCard(hand),
            4 => Hand::OnePair(hand),
            3 => {
                if *uniques.values().max().unwrap() + num_jokers == 3 {
                    Hand::ThreeKind(hand)
                } else {
                    Hand::TwoPair(hand)
                }
            }
            2 => {
                if *uniques.values().max().unwrap() + num_jokers == 4 {
                    Hand::FourKind(hand)
                } else {
                    Hand::FullHouse(hand)
                }
            }
            1 | 0 => Hand::FiveKind(hand),
            _ => unreachable!(),
        };

        Ok(hand)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

common::read_main!();

#[test]
fn example() {
    let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(solve(input), 5905);
}
