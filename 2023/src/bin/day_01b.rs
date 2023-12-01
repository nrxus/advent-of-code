use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    let digits: HashMap<&str, char> = HashMap::from_iter([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);

    input
        .trim()
        .lines()
        .map(|l| {
            let positions: Vec<_> = digits
                .keys()
                .flat_map(|&d| {
                    let mut ret = vec![];
                    let mut remaining = l;
                    let mut base_p = 0;
                    while let Some(p) = remaining.find(d) {
                        ret.push(base_p + p);
                        let new_base = p + 1;
                        remaining = &remaining[new_base..];
                        base_p += new_base;
                    }

                    ret.into_iter().map(move |p| (d, p))
                })
                .collect();

            let first = positions.iter().min_by_key(|(_, p)| p).unwrap().0;
            let last = positions.iter().max_by_key(|(_, p)| p).unwrap().0;
            let first = digits.get(first).cloned().unwrap();
            let last = digits.get(last).cloned().unwrap();

            String::from_iter([first, last]).parse::<u32>().unwrap()
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
msixonexch1twokjbdlhchqk1
";
    assert_eq!(solve(input), 266);
}
