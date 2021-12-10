fn solve(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                let expected = match c {
                    '(' | '{' | '[' | '<' => {
                        stack.push(c);
                        continue;
                    }
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    e => panic!("unexpected char: {}", e),
                };

                let opener = stack
                    .pop()
                    .expect("closer should never appear before opener");

                if opener != expected {
                    return None;
                }
            }

            let score = stack.into_iter().rev().fold(0, |score, opener| {
                let points = match opener {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };

                score * 5 + points
            });

            Some(score)
        })
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve(input), 288957);
    }
}

common::read_main!();
