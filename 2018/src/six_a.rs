mod coord_parser;

use common::extensions::{cart_product, AbsDiff, IteratorExt};
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> u16 {
    let coords = coord_parser::parse(input).unwrap();

    let min_x = coords.iter().map(|(c, _)| c).min().cloned().unwrap();
    let min_y = coords.iter().map(|(_, r)| r).min().cloned().unwrap();

    let max_x = coords.iter().map(|(c, _)| c).max().cloned().unwrap();
    let max_y = coords.iter().map(|(_, r)| r).max().cloned().unwrap();

    let mut grid: HashMap<(u16, u16), u16> = HashMap::with_capacity(coords.len());
    let mut infinite_points: HashSet<(u16, u16)> = HashSet::new();

    cart_product(min_x..=max_x, min_y..=max_y)
        .filter_map(|(c, r)| {
            coords
                .iter()
                .uniq_min_by_key(|(c1, r1)| c1.abs_diff(c) + r1.abs_diff(r))
                .cloned()
                .map(|closest| (c, r, closest))
        })
        .for_each(|(c, r, closest)| {
            if c == min_x || c == max_x || r == min_y || r == max_y {
                infinite_points.insert(closest);
            } else {
                *(grid.entry(closest).or_default()) += 1;
            }
        });

    grid.into_iter()
        .filter(|(k, _)| !infinite_points.contains(k))
        .map(|(_, v)| v)
        .max()
        .unwrap()
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

        assert_eq!(solve(input), 17);
    }
}

common::bootstrap!(6);
