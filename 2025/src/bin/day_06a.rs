fn solve(input: &str) -> u64 {
    let mut lines: Vec<_> = input.trim().lines().map(str::split_whitespace).collect();
    let Some(ops) = lines.pop() else { return 0 };
    let mut lines = lines
        .into_iter()
        .map(|line| line.map(|n| n.parse::<u64>().unwrap()));
    let Some(res) = lines.next() else { return 0 };
    let ops: Vec<_> = ops.collect();
    let mut res: Vec<_> = res.collect();

    lines.for_each(|line| {
        res.iter_mut()
            .zip(ops.iter())
            .zip(line)
            .for_each(|((r, o), n)| match *o {
                "+" => *r += n,
                "*" => *r *= n,
                _ => unreachable!(),
            });
    });

    res.into_iter().sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"
        ),
        4277556
    );
}
