fn solve(input: &str) -> u32 {
    let mut input = input.trim();
    let mut sum = 0;
    let mut mul_enabled = true;

    while let Some(index) = input.find("mul(") {
        if input.len() < 5 {
            break;
        }
        let Some((left, right)) = input.split_at_checked(index + 4) else {
            break;
        };
        input = right;

        match (left.rfind("don't()"), left.rfind("do()")) {
            (None, None) => {}
            (None, Some(_)) => mul_enabled = true,
            (Some(_), None) => mul_enabled = false,
            (Some(disable), Some(enable)) => mul_enabled = enable > disable,
        }

        if !mul_enabled {
            continue;
        }

        let Some((left, rest)) = right.split_once(',') else {
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
        solve(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
}
