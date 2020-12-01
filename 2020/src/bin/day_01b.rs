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
                .enumerate()
                .find_map(|(cursor, &expense_two)| {
                    expenses[cursor + 1..]
                        .iter()
                        .find(|&expense_three| expense_one + expense_two + expense_three == 2020)
                        .map(|&expense_three| expense_one * expense_two * expense_three)
                })
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

        assert_eq!(solve(input), 241861950);
    }
}

common::read_main!();
