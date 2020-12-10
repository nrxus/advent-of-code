fn solve(input: &str) -> u32 {
    let mut ratings: Vec<u32> = std::iter::once(0)
        .chain(input.trim().lines().map(|l| l.parse().unwrap()))
        .collect();

    ratings.sort_unstable();

    let mut diff_one = 0;
    let mut diff_three = 1;

    ratings
        .windows(2)
        .for_each(|window| match window[1] - window[0] {
            1 => diff_one += 1,
            3 => diff_three += 1,
            _ => {}
        });

    diff_one * diff_three
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve(input), 35);
    }

    #[test]
    fn example_two() {
        let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(solve(input), 220);
    }
}

common::read_main!();
