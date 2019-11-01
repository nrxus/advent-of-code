use common::extensions::cart_product;

fn power_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    (rack_id * y as i32 + serial) * rack_id / 100 % 10 - 5
}

fn solve(input: &str) -> Answer {
    let serial: i32 = input.trim().parse().unwrap();
    let grid: Vec<_> = (0..=(300 * 300))
        .map(|i| power_level(i % 300 + 1, i / 300 + 1, serial))
        .collect();
    let mut cache = grid.clone();

    let every_square_and_size = (2..301_usize).flat_map(|size| {
        cart_product(0..301 - size, 0..301 - size).map(move |(x, y)| (x, y, size))
    });

    let power_level = |x, y| grid[y * 300 + x];

    every_square_and_size
        .max_by_key(|&(x, y, size)| {
            let mut sum = cache[y * 300 + x];
            for dx in 0..size {
                sum += power_level(x + dx, y + size - 1);
            }
            for dy in 0..size - 1 {
                sum += power_level(x + size - 1, y + dy);
            }
            cache[y * 300 + x] = sum;
            sum
        })
        .map(|(x, y, size)| Answer(x + 1, y + 1, size))
        .unwrap()
}

#[derive(PartialEq, Eq, Debug)]
struct Answer(usize, usize, usize);

use std::fmt;

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test() {
        assert_eq!(solve("18"), Answer(90, 269, 16));
        assert_eq!(solve("42"), Answer(232, 251, 12));
    }
}

common::read_main!();
//common::bootstrap!(11);
