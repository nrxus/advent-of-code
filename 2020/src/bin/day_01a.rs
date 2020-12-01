fn solve(expenses: &str) -> u32 {
    let expenses: Vec<u32> = expenses
        .trim()
        .lines()
        .map(|e| e.parse().expect("failed to parse expense"))
        .collect();

    expenses
        .iter()
        .enumerate()
        .find_map(|(cursor, &expense_one)| {
            expenses[cursor + 1..]
                .iter()
                .find(|&expense_two| expense_one + expense_two == 2020)
                .map(|&expense_two| expense_one * expense_two)
        })
        .expect("did not find pair")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"1721
979
366
299
675
1456";

        assert_eq!(solve(input), 514579);
    }
}

common::read_main!();
