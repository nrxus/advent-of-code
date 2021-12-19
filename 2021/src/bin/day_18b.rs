use std::{
    fmt::{self, Formatter, Write},
    ops::Add,
    str::FromStr,
};

fn solve(input: &str) -> u32 {
    let snails: Vec<_> = input
        .trim()
        .lines()
        .map(SnailFish::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    (0..snails.len())
        .flat_map(|i| (0..snails.len()).filter_map(move |j| Some((i, j)).filter(|(i, j)| i != j)))
        .map(|(i, j)| (snails[i].clone() + snails[j].clone()))
        .map(|sum| sum.magnitude())
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct SnailFish {
    left: Element,
    right: Element,
}

#[derive(Debug, Clone)]
enum Element {
    Number(u8),
    Pair(Box<SnailFish>),
}

impl FromStr for SnailFish {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        expect_next(&mut chars, '[')?;

        let snail_fish = Self::inner_from_chars(&mut chars)?;

        expect_next(&mut chars, ']')?;

        Ok(snail_fish)
    }
}
impl SnailFish {
    pub fn reduce(mut self) -> Self {
        loop {
            if let Explosion::None = self.explode(0) {
                if let Split::None = self.split() {
                    break;
                }
            }
        }

        self
    }

    pub fn magnitude(&self) -> u32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    fn inner_from_chars(
        chars: &mut impl Iterator<Item = char>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let left = Element::from_chars(chars)?;

        expect_next(chars, ',')?;

        let right = Element::from_chars(chars)?;

        Ok(Self { left, right })
    }

    fn split(&mut self) -> Split {
        match self.left.split() {
            Split::Done => Split::Done,
            Split::None => self.right.split(),
        }
    }

    fn explode(&mut self, depth: usize) -> Explosion {
        match (&mut self.left, &mut self.right) {
            (Element::Number(left), Element::Number(right)) => {
                if depth >= 4 {
                    Explosion::Started(*left, *right)
                } else {
                    Explosion::None
                }
            }
            (Element::Number(left), Element::Pair(right)) => match right.explode(depth + 1) {
                Explosion::Started(l, r) => {
                    self.right = Element::Number(0);
                    *left += l;
                    Explosion::Right(r)
                }
                Explosion::Left(l) => {
                    *left += l;
                    Explosion::Done
                }
                explosion => explosion,
            },
            (Element::Pair(left), Element::Number(right)) => match left.explode(depth + 1) {
                Explosion::Started(l, r) => {
                    self.left = Element::Number(0);
                    *right += r;
                    Explosion::Left(l)
                }
                Explosion::Right(r) => {
                    *right += r;
                    Explosion::Done
                }
                explosion => explosion,
            },
            (Element::Pair(left), Element::Pair(right)) => match left.explode(depth + 1) {
                Explosion::None => match right.explode(depth + 1) {
                    Explosion::Started(l, r) => {
                        self.right = Element::Number(0);
                        left.explode_to_last_number(l);
                        Explosion::Right(r)
                    }
                    Explosion::Left(l) => {
                        left.explode_to_last_number(l);
                        Explosion::Done
                    }
                    explosion => explosion,
                },
                Explosion::Started(l, r) => {
                    self.left = Element::Number(0);
                    right.explode_to_first_number(r);
                    Explosion::Left(l)
                }
                Explosion::Right(r) => {
                    right.explode_to_first_number(r);
                    Explosion::Done
                }
                explosion => explosion,
            },
        }
    }

    fn explode_to_first_number(&mut self, right: u8) {
        match &mut self.left {
            Element::Number(n) => *n += right,
            Element::Pair(p) => p.explode_to_first_number(right),
        }
    }

    fn explode_to_last_number(&mut self, left: u8) {
        match &mut self.right {
            Element::Number(n) => *n += left,
            Element::Pair(p) => p.explode_to_last_number(left),
        }
    }
}

#[derive(Debug)]
enum Split {
    None,
    Done,
}

#[derive(Debug)]
enum Explosion {
    None,
    Started(u8, u8),
    Left(u8),
    Right(u8),
    Done,
}

impl Element {
    fn split(&mut self) -> Split {
        match self {
            Element::Number(n) => {
                if *n >= 10 {
                    *self = Element::Pair(Box::new(SnailFish {
                        left: Element::Number(*n / 2),
                        right: Element::Number((*n + 1) / 2),
                    }));
                    Split::Done
                } else {
                    Split::None
                }
            }
            Element::Pair(pair) => pair.split(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Element::Number(n) => *n as u32,
            Element::Pair(p) => p.magnitude(),
        }
    }

    fn from_chars(
        chars: &mut impl Iterator<Item = char>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match chars.next().ok_or_else(|| "missing element".to_owned())? {
            '[' => {
                let element = SnailFish::inner_from_chars(chars)?;
                expect_next(chars, ']')?;
                Ok(Element::Pair(Box::new(element)))
            }
            c => match c.to_digit(10) {
                Some(d) => Ok(Element::Number(d as u8)),
                None => Err("expected number".to_owned().into()),
            },
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SnailFish {
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(rhs)),
        }
        .reduce()
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;
        write!(f, "{}", self.left)?;
        f.write_char(',')?;
        write!(f, "{}", self.right)?;
        f.write_char(']')?;

        Ok(())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Pair(p) => write!(f, "{}", p),
        }
    }
}

fn expect_next(
    chars: &mut impl Iterator<Item = char>,
    expected: char,
) -> Result<(), Box<dyn std::error::Error>> {
    match chars.next() {
        Some(c) if c == expected => Ok(()),
        Some(c) => Err(format!("expected '{}' found '{}'", expected, c).into()),
        None => Err(format!("missing '{}'", expected).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!(solve(input), 3993);
    }
}

common::read_main!();
