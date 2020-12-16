use common::extensions::cart_product;

fn power_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    (rack_id * y as i32 + serial) * rack_id / 100 % 10 - 5
}

fn solve(input: &str) -> Coord {
    let serial: i32 = input.trim().parse().unwrap();
    let mut grid: [i32; 300 * 300] = [0; 300 * 300];
    grid.iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = power_level(i % 300 + 1, i / 300 + 1, serial));

    cart_product(0..297_usize, 0..297_usize)
        .max_by_key(|(x, y)| {
            cart_product(0..3, 0..3)
                .map(|(dx, dy)| (x + dx, y + dy))
                .map(|(x, y)| grid[y * 300 + x])
                .sum::<i32>()
        })
        .map(|(x, y)| Coord(x + 1, y + 1))
        .unwrap()
}

#[derive(PartialEq, Eq, Debug)]
struct Coord(usize, usize);

use std::fmt;

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test() {
        assert_eq!(solve("18"), Coord(33, 45));
        assert_eq!(solve("42"), Coord(21, 61));
    }
}

common::read_main!();
