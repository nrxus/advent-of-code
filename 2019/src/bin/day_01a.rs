fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .map(fuel_required)
        .sum()
}

fn fuel_required(weight: u32) -> u32 {
    weight / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }
}

common::read_main!();
