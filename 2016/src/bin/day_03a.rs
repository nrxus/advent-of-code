fn solve(design_document: &str) -> usize {
    design_document
        .trim()
        .lines()
        .map(|l| {
            let mut numbers = l.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            let mut numbers = [
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            ];
            numbers.sort();
            numbers
        })
        .filter(|numbers| numbers[0] + numbers[1] > numbers[2])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"5 10 25
25 13 13";
        assert_eq!(solve(input), 1);
    }
}

common::read_main!();
