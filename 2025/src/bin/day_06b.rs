fn solve(input: &str) -> u64 {
    let input = input.trim_end_matches('\n');
    let mut lines: Vec<_> = input.lines().collect();
    let operators = lines.pop().unwrap().split_whitespace();
    let width = lines[0].len();
    let mut transposed = String::with_capacity(input.len());
    for x in 0..width {
        let line: String = lines
            .iter()
            .map(|line| line.as_bytes()[x] as char)
            .collect();
        transposed.push_str(line.trim());
        transposed.push('\n');
    }

    transposed
        .split("\n\n")
        .zip(operators)
        .map(|(numbers, operator)| {
            let numbers = numbers.lines().map(|l| l.parse::<u64>().unwrap());
            match operator {
                "+" => numbers.sum::<u64>(),
                "*" => numbers.product(),
                _ => unreachable!(),
            }
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  "),
        3263827
    );
}
