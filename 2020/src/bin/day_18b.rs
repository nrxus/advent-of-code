fn solve(homework: &str) -> u64 {
    homework
        .trim()
        .lines()
        .map(|expression| {
            let mut numbers: Vec<Vec<u64>> = vec![vec![]];
            let mut operators: Vec<Operator> = vec![];

            expression.split_whitespace().for_each(|e| match e {
                "+" => operators.push(Operator::Sum),
                "*" => operators.push(Operator::Multiply),
                num => {
                    let num_start_parens = num.chars().take_while(|c| !c.is_digit(10)).count();
                    if num_start_parens > 0 {
                        numbers.extend(std::iter::repeat(vec![]).take(num_start_parens - 1));
                        numbers.push(vec![num[num_start_parens..].parse().unwrap()]);
                    } else {
                        let num_end_parens = num.chars().skip_while(|c| c.is_digit(10)).count();
                        let last_idx = numbers.len() - 1;
                        let last = &mut numbers[last_idx];
                        last.push(num[..num.len() - num_end_parens].parse().unwrap());

                        match operators.pop() {
                            Some(Operator::Sum) => {
                                let a = last.pop().unwrap();
                                let b = last.pop().unwrap_or(0);
                                last.push(a + b);
                            }
                            Some(Operator::Multiply) => {
                                operators.push(Operator::Multiply);
                            }
                            None => {}
                        };

                        for _ in 0..num_end_parens {
                            let to_multiply = numbers.pop().unwrap();
                            operators
                                .drain(operators.len() + 1 - to_multiply.len()..)
                                .for_each(|_| {});
                            let next: u64 = to_multiply.into_iter().product::<u64>();
                            let last_idx = numbers.len() - 1;
                            let last = &mut numbers[last_idx];
                            last.push(next);

                            match operators.pop() {
                                Some(Operator::Sum) => {
                                    let a = last.pop().unwrap();
                                    let next = match last.pop() {
                                        Some(b) => a + b,
                                        None => {
                                            operators.push(Operator::Sum);
                                            a
                                        }
                                    };
                                    last.push(next);
                                }
                                Some(Operator::Multiply) => {
                                    operators.push(Operator::Multiply);
                                }
                                None => {}
                            };
                        }
                    }
                }
            });

            debug_assert!(numbers.len() == 1);
            numbers.pop().unwrap().into_iter().product::<u64>()
        })
        .sum()
}

#[derive(Clone, Copy, Debug)]
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
        assert_eq!(solve(input), 231);
    }

    #[test]
    fn example_two() {
        let input = r"1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(solve(input), 51);
    }

    #[test]
    fn example_three() {
        let input = r"2 * 3 + (4 * 5)";
        assert_eq!(solve(input), 46);
    }

    #[test]
    fn example_four() {
        let input = r"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(solve(input), 1445);
    }

    #[test]
    fn example_five() {
        let input = r"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(solve(input), 669060);
    }

    #[test]
    fn example_six() {
        let input = r"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(solve(input), 23340);
    }

    #[test]
    fn example_seven() {
        let input = r"6 + ((4 * 9 + 6 + 5 * 5 * 7) * 5 * 3) + 7";
        assert_eq!(solve(input), 42013);
    }

    #[test]
    fn example_sum() {
        let input = r"2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(solve(input), 46 + 1445 + 669060 + 23340);
    }
}

common::read_main!();
