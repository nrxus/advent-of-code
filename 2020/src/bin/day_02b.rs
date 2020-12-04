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
                Regex::new(r"(?P<pos_1>\d+)-(?P<pos_2>\d+) (?P<goal>.): (?P<password>.+)").unwrap();
        }

        let captures = RE.captures(entry).unwrap();
        let pos_1 = captures.name("pos_1").unwrap().as_str().parse().unwrap();
        let pos_2 = captures.name("pos_2").unwrap().as_str().parse().unwrap();
        let goal = captures.name("goal").unwrap().as_str().parse().unwrap();
        let password = captures.name("password").unwrap().as_str();

        PasswordEntry {
            policy: Policy {
                positions: [pos_1, pos_2],
                goal,
            },
            password,
        }
    }
}

struct Policy {
    goal: char,
    positions: [usize; 2],
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
        let mut chars = example.chars();
        let one = chars.nth(self.positions[0] - 1).unwrap();
        let two = chars
            .nth(self.positions[1] - self.positions[0] - 1)
            .unwrap();

        (one == self.goal) ^ (two == self.goal)
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

        assert_eq!(solve(input), 1);
    }
}

common::read_main!();
