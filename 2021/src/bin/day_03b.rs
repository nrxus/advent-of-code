use bitvec::{field::BitField, order::Lsb0, prelude::BitVec};

fn solve(input: &str) -> u32 {
    let input = input.trim();

    let numbers: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let number = u64::from_str_radix(line, 2).unwrap();
            let mut number = BitVec::from_element(number);
            number.truncate(line.len());
            number.reverse();
            number
        })
        .collect();

    let oxygen_number = extract_rating(numbers.clone(), |ord| ord != std::cmp::Ordering::Less);
    let co2_number = extract_rating(numbers, |ord| ord == std::cmp::Ordering::Less);

    oxygen_number * co2_number
}

fn extract_rating(
    mut numbers: Vec<BitVec<u64, Lsb0>>,
    matcher: impl Fn(std::cmp::Ordering) -> bool,
) -> u32 {
    assert!(!numbers.is_empty());

    let width = numbers[0].len();

    for bit_index in 0..width {
        let num_ones = numbers.iter().filter(|number| number[bit_index]).count();
        let expected = matcher((num_ones * 2).cmp(&numbers.len()));
        numbers.retain(|number| number[bit_index] == expected);
        if numbers.len() == 1 {
            break;
        }
    }

    let mut oxygen_number = numbers.pop().unwrap();
    oxygen_number.reverse();
    oxygen_number.load_be()
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
        assert_eq!(solve(input), 230);
    }
}

common::read_main!();
