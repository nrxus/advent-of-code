fn solve(input: &str) -> String {
    let mut password = Password(input.trim().to_string()).next().unwrap();

    while !password.meets_requirement() {
        password = password.next().expect("no valid password found");
    }

    password = password.next().expect("no valid password found");

    while !password.meets_requirement() {
        password = password.next().expect("no valid password found");
    }

    password.0
}

#[derive(Debug)]
struct Password(String);

impl Password {
    fn next(mut self) -> Option<Password> {
        if self.0 == "zzzzzzzz" {
            return None;
        }

        unsafe {
            for c in self.0.as_mut_vec().iter_mut().rev() {
                if *c == b'z' {
                    *c = b'a';
                } else {
                    *c += 1;
                    break;
                }
            }
        }

        Some(self)
    }

    fn meets_requirement(&self) -> bool {
        let password = &self.0;
        let has_straight = password
            .as_bytes()
            .windows(3)
            .any(|window| window[0] == window[1] - 1 && window[1] == window[2] - 1);

        if !has_straight {
            return false;
        }

        let has_confusing_char =
            password.contains('i') || password.contains('o') || password.contains('l');

        if has_confusing_char {
            return false;
        }

        let mut last_was_pair = false;

        let at_least_two_pairs = password
            .as_bytes()
            .windows(2)
            .filter(|window| {
                if last_was_pair {
                    last_was_pair = false;
                } else {
                    last_was_pair = window[0] == window[1];
                }
                last_was_pair
            })
            .take(2)
            .count()
            == 2;

        at_least_two_pairs
    }
}

common::read_main!();
