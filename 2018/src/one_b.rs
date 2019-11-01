use common::{bootstrap, sanitize};

use std::collections::HashSet;

bootstrap!(1);

fn solve(input: &str) -> i32 {
    sanitize::to_i32s(input)
        .cycle()
        .scan(State::default(), |state, x| {
            if state.is_done() {
                None
            } else {
                state.add_number(x);
                Some(state.sum)
            }
        })
        .last()
        .unwrap()
}

#[derive(Default, Debug)]
struct State {
    partial_sums: HashSet<i32>,
    sum: i32,
}

impl State {
    fn is_done(&self) -> bool {
        self.partial_sums.contains(&self.sum)
    }

    fn add_number(&mut self, num: i32) {
        let sum = self.sum + num;
        self.partial_sums.insert(self.sum);
        self.sum = sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            solve(
                r#"+1
-1"#
            ),
            0
        );

        assert_eq!(
            solve(
                r#"+3
+3
+4
-2
-4"#
            ),
            10
        );

        assert_eq!(
            solve(
                r#"-6
+3
+8
+5
-6"#
            ),
            5
        );

        assert_eq!(
            solve(
                r#"+7
+7
-2
-7
-4"#
            ),
            14
        );
    }
}
