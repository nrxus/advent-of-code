use std::cmp::Ordering;

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|report| {
            let original_report: Vec<_> = report
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();

            if is_safe(&original_report) {
                return true;
            }

            (0..original_report.len()).any(|to_remove| {
                let mut report = original_report.clone();
                report.remove(to_remove);

                is_safe(&report)
            })
        })
        .count()
}

fn is_safe(report: &[u16]) -> bool {
    let expected_order = report[0].cmp(&report[1]);
    if expected_order == Ordering::Equal {
        return false;
    }

    report.windows(2).all(|pair| {
        let [a, b] = pair.try_into().unwrap();
        a.cmp(&b) == expected_order && a.abs_diff(b) <= 3
    })
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
        4
    );
}
