use bitvec::{field::BitField, prelude::BitVec};

fn solve(input: &str) -> u32 {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();

    let mut columns: Vec<BitVec> = vec![BitVec::default(); width];

    let height = input
        .trim()
        .lines()
        .inspect(|line| {
            line.chars().enumerate().for_each(|(i, bit)| {
                let bit = match bit {
                    '0' => false,
                    '1' => true,
                    b => panic!("unexpected bit: {}", b),
                };
                columns[i].push(bit)
            })
        })
        .count();

    let gamma_bits: BitVec = columns
        .into_iter()
        .map(|c| c.count_ones() > height / 2)
        .rev()
        .collect();
    let gamma_rate: u32 = gamma_bits.load_be();

    let epsilon_bits: BitVec = !gamma_bits;
    let epsilon_rate: u32 = epsilon_bits.load_be();

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(solve(input), 198);
    }
}

common::read_main!();
