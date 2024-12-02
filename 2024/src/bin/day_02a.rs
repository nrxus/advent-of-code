use std::cmp::Ordering;

fn solve(input: &str) -> usize {
    let safety_check = |a: u16, b: u16| a.abs_diff(b) <= 3;

    input
        .trim()
        .lines()
        .filter(|report| {
            let report: Vec<_> = report
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();

            let mut pairs = report.windows(2);
            let [a, b] = pairs.next().unwrap().try_into().unwrap();
            let ord = a.cmp(&b);
            if ord == Ordering::Equal {
                return false;
            }

            if !safety_check(a, b) {
                return false;
            }

            pairs.all(|pair| {
                let [a, b] = pair.try_into().unwrap();
                a.cmp(&b) == ord && safety_check(a, b)
            })
        })
        .count()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
        ),
        2
    );
}
