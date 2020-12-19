fn solve(homework: &str) -> u64 {
    homework
        .trim()
        .lines()
        .map(|expression| {
            let mut numbers: Vec<u64> = vec![0];
            let mut operators: Vec<Operator> = vec![Operator::Sum];

            expression.split_whitespace().for_each(|e| match e {
                "+" => operators.push(Operator::Sum),
                "*" => operators.push(Operator::Multiply),
                num => {
                    let num_start_parens = num.chars().take_while(|c| !c.is_digit(10)).count();
                    if num_start_parens > 0 {
                        numbers.extend(std::iter::repeat(0).take(num_start_parens - 1));
                        operators
                            .extend(std::iter::repeat(Operator::Sum).take(num_start_parens - 1));
                        numbers.push(num[num_start_parens..].parse().unwrap());
                    } else {
                        let num_end_parens = num.chars().skip_while(|c| c.is_digit(10)).count();
                        numbers.push(num[..num.len() - num_end_parens].parse().unwrap());

                        for _ in 0..=num_end_parens {
                            let a = numbers.pop().unwrap();
                            let b = numbers.pop().unwrap();
                            numbers.push(match operators.pop().unwrap() {
                                Operator::Sum => a + b,
                                Operator::Multiply => a * b,
                            });
                        }
                    }
                }
            });

            debug_assert!(numbers.len() == 1);
            numbers.pop().unwrap()
        })
        .sum()
}

#[derive(Clone, Copy)]
enum Operator {
    Sum,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(solve(input), 71);
    }

    #[test]
    fn example_two() {
        let input = r"1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(solve(input), 51);
    }

    #[test]
    fn example_three() {
        let input = r"2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(solve(input), 26 + 437 + 12240 + 13632);
    }
}

common::read_main!();
