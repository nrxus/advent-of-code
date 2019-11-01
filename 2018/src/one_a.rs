use common::{bootstrap, sanitize};

bootstrap!(1);

fn solve(input: &str) -> i32 {
    sanitize::to_i32s(input).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            solve(
                r#"+1
+1
+1"#
            ),
            3
        );

        assert_eq!(
            solve(
                r#"+1
+1
-2"#
            ),
            0
        );

        assert_eq!(
            solve(
                r#"-1
-2
-3"#
            ),
            -6
        );
    }
}
