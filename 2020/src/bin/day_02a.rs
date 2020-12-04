fn solve(password_db: &str) -> usize {
    password_db
        .trim()
        .lines()
        .map(PasswordEntry::from)
        .filter(PasswordEntry::is_valid)
        .count()
}

impl<'p> From<&'p str> for PasswordEntry<'p> {
    fn from(entry: &'p str) -> Self {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<goal>.): (?P<password>.+)").unwrap();
        }

        let captures = RE.captures(entry).unwrap();
        let min = captures.name("min").unwrap().as_str().parse().unwrap();
        let max = captures.name("max").unwrap().as_str().parse().unwrap();
        let goal = captures.name("goal").unwrap().as_str().parse().unwrap();
        let password = captures.name("password").unwrap().as_str();

        PasswordEntry {
            policy: Policy { min, max, goal },
            password,
        }
    }
}

struct Policy {
    goal: char,
    min: usize,
    max: usize,
}

struct PasswordEntry<'p> {
    policy: Policy,
    password: &'p str,
}

impl PasswordEntry<'_> {
    fn is_valid(&self) -> bool {
        self.policy.matches(self.password)
    }
}

impl Policy {
    fn matches(&self, example: &str) -> bool {
        let count = example
            .chars()
            .filter(|c| *c == self.goal)
            .take(self.max + 1)
            .count();
        count >= self.min && count <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        assert_eq!(solve(input), 2);
    }
}

common::read_main!();
