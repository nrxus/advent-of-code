fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .map(fuel_required)
        .sum()
}

fn fuel_required(weight: u32) -> u32 {
    match (weight / 3).checked_sub(2) {
        Some(fuel) => fuel + fuel_required(fuel),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(1969), 966);
        assert_eq!(fuel_required(100756), 50346);
    }
}

common::read_main!();
