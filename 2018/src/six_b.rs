mod coord_parser;

use common::extensions::{cart_product, AbsDiff};

fn solve(input: &str) -> usize {
    region(input, 10_000)
}

fn region(input: &str, max_distance: u16) -> usize {
    let coords = coord_parser::parse(input).unwrap();

    let min_x = coords.iter().map(|(c, _)| c).min().cloned().unwrap();
    let min_y = coords.iter().map(|(_, r)| r).min().cloned().unwrap();

    let max_x = coords.iter().map(|(c, _)| c).max().cloned().unwrap();
    let max_y = coords.iter().map(|(_, r)| r).max().cloned().unwrap();

    let x_start = max_x as i16 - max_distance as i16;
    let x_end = min_x as i16 + max_distance as i16;
    let y_start = max_y as i16 - max_distance as i16;
    let y_end = min_y as i16 + max_distance as i16;

    cart_product(x_start..=x_end, y_start..=y_end)
        .map(|(c, r)| {
            coords
                .iter()
                .map(|(c1, r1)| c1.abs_diff(c) + r1.abs_diff(r))
                .map(u32::from)
                .sum::<u32>()
        })
        .filter(|&d| d < u32::from(max_distance))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        assert_eq!(region(input, 32), 16);
    }
}

//common::read_main!();
common::bootstrap!(6);
