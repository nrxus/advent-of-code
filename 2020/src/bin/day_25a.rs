fn solve(public_keys: &str) -> u64 {
    let subject_number: u64 = 7;
    let mut public_keys = public_keys.trim().lines().map(|k| k.parse().unwrap());
    let public_a: u64 = public_keys.next().unwrap();
    let public_b: u64 = public_keys.next().unwrap();

    let mut loop_size_a = 0;
    let mut value = 1;
    while value != public_a {
        value = (value * subject_number) % 20201227;
        loop_size_a += 1;
    }

    let mut encryption_key = 1;
    let subject_number = public_b;
    (0..loop_size_a).for_each(|_| encryption_key = (encryption_key * subject_number) % 20201227);

    encryption_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"17807724
5764801";
        assert_eq!(solve(input), 14897079);
    }
}

common::read_main!();
