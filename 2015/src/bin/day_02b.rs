use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Dimensions>().unwrap())
        .map(|d| d.ribbon_length())
        .sum()
}

struct Dimensions(u32, u32, u32);

impl Dimensions {
    fn ribbon_length(&self) -> u32 {
        let ribbon = self.0 * self.1 * self.2;
        let lw = self.0 + self.1;
        let wh = self.1 + self.2;
        let hl = self.2 + self.0;
        let sides = 2 * std::cmp::min(std::cmp::min(lw, wh), hl);

        ribbon + sides
    }
}

impl FromStr for Dimensions {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
        }

        let captured_int = |caps: &regex::Captures, i| -> Result<u32, Self::Err> {
            caps.get(i)
                .ok_or_else(|| "capture not found")?
                .as_str()
                .parse()
                .map_err(|e: std::num::ParseIntError| e.into())
        };

        let caps = RE.captures(input).ok_or_else(|| "failed to capture line")?;
        Ok(Dimensions(
            captured_int(&caps, 1)?,
            captured_int(&caps, 2)?,
            captured_int(&caps, 3)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singles() {
        assert_eq!(solve("1x1x10"), 14);
        assert_eq!(solve("2x3x4"), 34);
    }

    #[test]
    fn test_many() {
        let input = r"2x3x4
1x1x10
";
        assert_eq!(solve(input), 48);
    }
}

common::read_main!();
