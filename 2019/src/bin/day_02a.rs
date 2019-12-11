use intcode::{Intcode, Machine};

fn solve(input: &str) -> i64 {
    let mut codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    codes[1] = Intcode(12);
    codes[2] = Intcode(2);

    first_after_run(codes)
}

fn first_after_run(codes: Vec<Intcode>) -> i64 {
    Machine::new(codes)
        .run_to_halt()
        .expect("did not halt")
        .get(0)
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            first_after_run([1, 0, 0, 0, 99].iter().map(|&i| Intcode(i)).collect()),
            2
        );
        assert_eq!(
            first_after_run([2, 3, 0, 3, 99].iter().map(|&i| Intcode(i)).collect()),
            2
        );
        assert_eq!(
            first_after_run([2, 4, 4, 5, 99, 0].iter().map(|&i| Intcode(i)).collect()),
            2
        );
        assert_eq!(
            first_after_run(
                [1, 1, 1, 4, 99, 5, 6, 0, 99]
                    .iter()
                    .map(|&i| Intcode(i))
                    .collect()
            ),
            30
        );
        assert_eq!(
            first_after_run(
                [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
                    .iter()
                    .map(|&i| Intcode(i))
                    .collect()
            ),
            3500
        );
    }
}

common::read_main!();
