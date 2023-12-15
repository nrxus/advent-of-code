fn solve(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .map(|r| {
            r.bytes().fold(0, |mut value, next| {
                value += next as u32;
                value *= 17;
                value %= 256;
                value
            })
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(solve(input), 1320);
}
