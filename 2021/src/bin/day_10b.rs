fn solve(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| Brace::from_char(c).unwrap()))
        .filter_map(Brace::find_incomplete)
        .map(|unpaired| {
            unpaired
                .into_iter()
                .rev()
                .fold(0, |score, opener| score * 5 + opener.points())
        })
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[derive(Clone, Copy)]
enum Opener {
    Paren = 1,
    Bracket = 2,
    Curly = 3,
    Angled = 4,
}

#[derive(Clone, Copy)]
enum Closer {
    Paren,
    Bracket,
    Curly,
    Angled,
}

enum Brace {
    Opener(Opener),
    Closer(Closer),
}

impl Brace {
    pub fn from_char(c: char) -> Result<Self, Box<dyn std::error::Error>> {
        let brace = match c {
            '(' => Brace::Opener(Opener::Paren),
            '{' => Brace::Opener(Opener::Curly),
            '[' => Brace::Opener(Opener::Bracket),
            '<' => Brace::Opener(Opener::Angled),
            ')' => Brace::Closer(Closer::Paren),
            ']' => Brace::Closer(Closer::Bracket),
            '}' => Brace::Closer(Closer::Curly),
            '>' => Brace::Closer(Closer::Angled),
            e => return Err(format!("invalid brace char: {}", e).into()),
        };

        Ok(brace)
    }

    pub fn find_incomplete(mut braces: impl Iterator<Item = Self>) -> Option<Vec<Opener>> {
        braces.try_fold(vec![], |mut openers, brace| {
            match brace {
                Brace::Opener(opener) => openers.push(opener),
                Brace::Closer(closer) => {
                    let opener = openers.pop()?;
                    if !opener.matchers_closer(closer) {
                        return None;
                    }
                }
            }

            Some(openers)
        })
    }
}

impl Opener {
    pub fn points(self) -> u64 {
        self as u64
    }

    pub fn matchers_closer(self, closer: Closer) -> bool {
        matches!(
            (self, closer),
            (Opener::Paren, Closer::Paren)
                | (Opener::Bracket, Closer::Bracket)
                | (Opener::Curly, Closer::Curly)
                | (Opener::Angled, Closer::Angled)
        )
    }
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
