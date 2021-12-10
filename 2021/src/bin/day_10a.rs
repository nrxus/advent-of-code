// 367971

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut stack = vec![];

            for c in line.chars() {
                let (expected, points) = match c {
                    '(' | '{' | '[' | '<' => {
                        stack.push(c);
                        continue;
                    }
                    ')' => ('(', 3),
                    ']' => ('[', 57),
                    '}' => ('{', 1197),
                    '>' => ('<', 25137),
                    e => panic!("unexpected char: {}", e),
                };

                let opener = stack
                    .pop()
                    .expect("closer should never appear before opener");

                if opener != expected {
                    return points;
                }
            }

            0
        })
        .sum()
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
        assert_eq!(solve(input), 26397);
    }
}

common::read_main!();
