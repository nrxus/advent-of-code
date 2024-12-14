fn solve(input: &str) -> u64 {
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
            let goal: (u64, u64) = (px.parse().unwrap(), py.parse().unwrap());
            let goal = (goal.0 + 10000000000000_u64, goal.1 + 10000000000000_u64);
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

fn parse_button(a: &str) -> (u64, u64) {
    let (x, y) = a.split_once(',').unwrap();
    let (_, x) = x.split_once("+").unwrap();
    let (_, y) = y.split_once("+").unwrap();
    let x: u64 = x.parse().unwrap();
    let y: u64 = y.parse().unwrap();
    (x, y)
}

common::read_main!();
