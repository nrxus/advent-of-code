use std::{
    collections::HashSet,
    fmt::{self, Write},
    str::FromStr,
};

fn solve(input: &str) -> usize {
    let (rules, image) = input.split_once("\n\n").unwrap();
    assert!(rules.len() == 512);

    let rules = {
        let mut bits = [Color::Light; 512];

        rules
            .chars()
            .enumerate()
            .try_for_each(|(i, c)| match Color::from_char(c) {
                Ok(color) => {
                    bits[i] = color;
                    Ok(())
                }
                Err(e) => Err(e),
            })
            .unwrap();

        bits
    };

    let image = (0..50).fold(Image::from_str(image).unwrap(), |image, _| {
        image.enhance(&rules)
    });

    match image {
        Image::DarkBack { light } => light.len(),
        Image::LightBack { .. } => panic!("infinite light pixels!"),
    }
}

#[derive(Debug)]
enum Image {
    DarkBack { light: HashSet<(i16, i16)> },
    LightBack { dark: HashSet<(i16, i16)> },
}

impl Image {
    pub fn enhance(self, rules: &[Color; 512]) -> Image {
        match self {
            Image::DarkBack { light } => {
                let expanded = expand(&light);
                let next_back = rules[0];
                let next = expanded
                    .into_iter()
                    .filter(|p| {
                        let bits: String = neighbors(*p)
                            .into_iter()
                            .map(|pos| char::from_digit(light.contains(&pos) as u32, 2).unwrap())
                            .collect();

                        let index = usize::from_str_radix(&bits, 2).unwrap();
                        let color = rules[index];

                        color != next_back
                    })
                    .collect();

                match next_back {
                    Color::Light => Image::LightBack { dark: next },
                    Color::Dark => Image::DarkBack { light: next },
                }
            }
            Image::LightBack { dark } => {
                let expanded = expand(&dark);
                let next_back = rules[254];

                let next = expanded
                    .into_iter()
                    .filter(|p| {
                        let bits: String = neighbors(*p)
                            .into_iter()
                            .map(|pos| char::from_digit(!dark.contains(&pos) as u32, 2).unwrap())
                            .collect();

                        let index = usize::from_str_radix(&bits, 2).unwrap();
                        let color = rules[index];

                        color != next_back
                    })
                    .collect();

                match next_back {
                    Color::Light => Image::LightBack { dark: next },
                    Color::Dark => Image::DarkBack { light: next },
                }
            }
        }
    }
}

fn expand(pixels: &HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
    pixels.iter().flat_map(|&pixel| neighbors(pixel)).collect()
}

fn neighbors((x, y): (i16, i16)) -> [(i16, i16); 9] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Light,
    Dark,
}

impl FromStr for Image {
    type Err = Box<dyn std::error::Error>;

    fn from_str(image: &str) -> Result<Self, Self::Err> {
        let points = image
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    Color::from_char(c)
                        .map(|c| match c {
                            Color::Dark => None,
                            Color::Light => Some((x as i16, y as i16)),
                        })
                        .transpose()
                })
            })
            .collect::<Result<HashSet<_>, Self::Err>>()?;

        Ok(Image::DarkBack { light: points })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pixels, back_c, fore_c) = match self {
            Image::DarkBack { light } => (light, '.', '#'),
            Image::LightBack { dark } => (dark, '#', '.'),
        };

        let min_x = *pixels.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *pixels.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *pixels.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *pixels.iter().map(|(_, y)| y).max().unwrap();

        (min_y..=max_y).try_for_each(|y| {
            (min_x..=max_x).try_for_each(|x| {
                let c = if pixels.contains(&(x, y)) {
                    fore_c
                } else {
                    back_c
                };

                f.write_char(c)
            })?;

            f.write_str("\n")
        })
    }
}

impl Color {
    pub fn from_char(c: char) -> Result<Self, Box<dyn std::error::Error>> {
        match c {
            '.' => Ok(Color::Dark),
            '#' => Ok(Color::Light),
            e => Err(format!("unexpected character in image: {}", e).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ignore because the test is slow
    #[ignore]
    fn example() {
        let input = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        assert_eq!(solve(input), 3351);
    }
}

common::read_main!();
