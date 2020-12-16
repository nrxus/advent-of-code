use enum_map::{Enum, EnumMap};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PotState {
    Empty,
    Full,
}

impl PotState {
    fn new(s: u8) -> Self {
        if s == b'#' {
            PotState::Full
        } else {
            PotState::Empty
        }
    }
}

impl Default for PotState {
    fn default() -> Self {
        PotState::Empty
    }
}

#[derive(Debug)]
struct GenDelta([PotState; 5]);

fn solve(input: &str) -> isize {
    let mut lines = input.lines();
    let mut generation = lines.next().map(initial).unwrap();
    let mut previous = generation
        .iter()
        .map(|(_, s)| *s)
        .skip_while(|&s| s == PotState::Empty)
        .collect::<Vec<_>>();
    let deltas = deltas(lines.skip(1));

    let mut final_iteration = None;
    let num_generations = 50_000_000_000_u64;

    for i in 0..num_generations {
        let min = generation[0].0;

        let left_padding = generation
            .iter()
            .take_while(|&&(_, s)| s == PotState::Empty)
            .count() as isize;

        let right_padding = generation
            .iter()
            .rev()
            .take_while(|&&(_, s)| s == PotState::Empty)
            .count() as isize;
        let max = generation[generation.len() - 1].0;

        if left_padding < 4 {
            let mut padding: Vec<_> = (1..=4 - left_padding)
                .map(|i| (min - i, PotState::Empty))
                .rev()
                .collect();
            padding.extend(generation);
            generation = padding;
        }

        if right_padding < 4 {
            let padding: Vec<_> = (1..=4 - right_padding)
                .map(|i| (max + i, PotState::Empty))
                .collect();
            generation.extend(padding);
        }

        generation = generation
            .windows(5)
            .map(|w| {
                let step = GenDelta([w[0].1, w[1].1, w[2].1, w[3].1, w[4].1]);
                (w[2].0, deltas[step])
            })
            .collect();

        let current = generation
            .iter()
            .map(|(_, s)| *s)
            .skip_while(|&s| s == PotState::Empty)
            .collect::<Vec<_>>();

        if current == previous {
            final_iteration = Some((i, generation[0].0 - min));
            break;
        } else {
            previous = current;
        }
    }

    let delta = final_iteration
        .map(|(i, delta)| delta * (num_generations - i - 1) as isize)
        .unwrap_or(0);

    generation
        .into_iter()
        .filter_map(|(i, s)| match s {
            PotState::Empty => None,
            PotState::Full => Some(i),
        })
        .map(|i| i + delta)
        .sum()
}

fn deltas<'a>(lines: impl Iterator<Item = &'a str>) -> EnumMap<GenDelta, PotState> {
    let regex = Regex::new(r"(?P<prev>[#|\.]*) => (?P<next>[#\.])").unwrap();
    lines
        .map(|i| {
            let caps = regex.captures(i).unwrap();
            let prev = caps.name("prev").unwrap().as_str().as_bytes();
            let next = caps.name("next").unwrap().as_str().as_bytes();
            let next = PotState::new(next[0]);
            let first = PotState::new(prev[0]);
            let second = PotState::new(prev[1]);
            let third = PotState::new(prev[2]);
            let fourth = PotState::new(prev[3]);
            let fifth = PotState::new(prev[4]);
            (GenDelta([first, second, third, fourth, fifth]), next)
        })
        .fold(EnumMap::new(), |mut deltas, (d, s)| {
            deltas[d] = s;
            deltas
        })
}

fn initial(input: &str) -> Vec<(isize, PotState)> {
    let regex = Regex::new(r"initial state: (?P<initial>[#|\.]*)").unwrap();
    let caps = regex.captures(input).unwrap();
    let initial = caps.name("initial").unwrap().as_str();
    initial
        .bytes()
        .enumerate()
        .map(|(i, s)| (i as isize, PotState::new(s)))
        .collect()
}

impl<T> Enum<T> for GenDelta {
    type Array = [T; 32];
    const POSSIBLE_VALUES: usize = 32;

    #[inline]
    fn slice(array: &Self::Array) -> &[T] {
        array
    }

    #[inline]
    fn slice_mut(array: &mut Self::Array) -> &mut [T] {
        array
    }

    #[inline]
    fn from_usize(value: usize) -> Self {
        let one = if value % 2 == 0 {
            PotState::Empty
        } else {
            PotState::Full
        };

        let value = value / 2;
        let two = if value % 2 == 0 {
            PotState::Empty
        } else {
            PotState::Full
        };

        let value = value / 2;
        let three = if value % 2 == 0 {
            PotState::Empty
        } else {
            PotState::Full
        };

        let value = value / 2;
        let four = if value % 2 == 0 {
            PotState::Empty
        } else {
            PotState::Full
        };

        let value = value / 2;
        let five = if value % 2 == 0 {
            PotState::Empty
        } else {
            PotState::Full
        };

        GenDelta([one, two, three, four, five])
    }

    #[inline]
    fn to_usize(self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, s)| match s {
                PotState::Empty => 0,
                PotState::Full => 2_usize.pow(i as u32),
            })
            .sum()
    }

    #[inline]
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> Self::Array {
        [
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
                PotState::Full,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Empty,
            ])),
            f(GenDelta([
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Full,
                PotState::Full,
            ])),
        ]
    }
}

common::read_main!();
