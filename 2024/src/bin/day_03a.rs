fn solve(input: &str) -> u32 {
    let mut input = input.trim();
    let mut sum = 0;
    while let Some(index) = input.find("mul(") {
        let Some(i) = input.get(index + 4..) else {
            break;
        };

        input = i;
        let Some((left, rest)) = i.split_once(',') else {
            break;
        };

        if left.len() < 1 || left.len() > 3 {
            continue;
        }
        let Ok(left) = left.parse::<u32>() else {
            continue;
        };
        let Some((right, rest)) = rest.split_once(')') else {
            continue;
        };

        if right.len() < 1 || right.len() > 3 {
            continue;
        }
        let Ok(right) = right.parse::<u32>() else {
            continue;
        };

        input = rest;
        sum += left * right;
    }
    sum
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    );
}
