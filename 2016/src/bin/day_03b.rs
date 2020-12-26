fn solve(design_document: &str) -> usize {
    let mut numbers = design_document
        .trim()
        .lines()
        .flat_map(|l| l.trim().split_whitespace())
        .map(|n| n.parse::<u32>().unwrap());

    std::iter::from_fn(|| {
        Some((
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
            numbers.next()?,
        ))
    })
    .flat_map(|nine| {
        let mut t1 = [nine.0, nine.3, nine.6];
        let mut t2 = [nine.1, nine.4, nine.7];
        let mut t3 = [nine.2, nine.5, nine.8];
        t1.sort();
        t2.sort();
        t3.sort();
        vec![t1, t2, t3]
    })
    .filter(|numbers| numbers[0] + numbers[1] > numbers[2])
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"5 13 22
10 13 5
25 25 8";
        assert_eq!(solve(input), 1);
    }
}

common::read_main!();
