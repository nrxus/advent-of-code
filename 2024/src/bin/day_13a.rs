fn solve(input: &str) -> u32 {
    input
        .trim()
        .split("\n\n")
        .filter_map(|rules| {
            let (a, rules) = rules.split_once("\n").unwrap();
            let a = parse_button(a);
            let (b, prize) = rules.split_once("\n").unwrap();
            let b = parse_button(b);
            let (px, py) = prize.split_once(',').unwrap();
            let (_, px) = px.split_once("=").unwrap();
            let (_, py) = py.split_once("=").unwrap();
            let goal: (u32, u32) = (px.parse().unwrap(), py.parse().unwrap());
            let axby = a.0 * b.1;
            let aybx = a.1 * b.0;

            let (top, bottom) = if axby > aybx {
                let bottom = axby - aybx;
                let top = goal
                    .1
                    .checked_mul(a.0)
                    .unwrap()
                    .checked_sub(goal.0.checked_mul(a.1).unwrap())?;
                (top, bottom)
            } else if axby < aybx {
                let bottom = aybx - axby;
                let top = goal
                    .0
                    .checked_mul(a.1)
                    .unwrap()
                    .checked_sub(goal.1.checked_mul(a.0).unwrap())?;
                (top, bottom)
            } else {
                panic!("NOOO")
            };

            if top % bottom != 0 {
                return None;
            }

            let b_presses = top / bottom;
            let remaining_x = goal.0 - (b_presses * b.0);
            if remaining_x % a.0 != 0 {
                return None;
            }

            let a_presses = remaining_x / a.0;

            Some(b_presses + 3 * a_presses)
        })
        .sum()
}

fn parse_button(a: &str) -> (u32, u32) {
    let (x, y) = a.split_once(',').unwrap();
    let (_, x) = x.split_once("+").unwrap();
    let (_, y) = y.split_once("+").unwrap();
    let x: u32 = x.parse().unwrap();
    let y: u32 = y.parse().unwrap();
    (x, y)
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
        ),
        480
    );
}
