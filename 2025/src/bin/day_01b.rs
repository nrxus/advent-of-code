fn solve(input: &str) -> usize {
    const START_POS: u16 = 50;
    const MAX_NUM: u16 = 100;

    input
        .trim()
        .lines()
        .map(|instruction| {
            let number: u16 = instruction[1..]
                .parse::<u16>()
                .expect("expected number of clicks");
            let direction = &instruction[0..1];
            (number, direction)
        })
        .scan(START_POS, |position, (number, direction)| {
            let mut num_clicks = (number / MAX_NUM) as usize; // it will click no matter what

            let number = number % MAX_NUM;
            if number == 0 {
                return Some(num_clicks);
            }

            match direction {
                "L" => {
                    let (p, underflow) = position.overflowing_sub(number);
                    *position = if underflow {
                        if *position != 0 {
                            num_clicks += 1;
                        }

                        p.wrapping_add(MAX_NUM)
                    } else {
                        p
                    };
                }
                "R" => {
                    let p = *position + number;
                    if p > MAX_NUM {
                        num_clicks += 1;
                    }

                    *position = p % MAX_NUM;
                }
                _ => {
                    panic!("expected L or R")
                }
            }

            Some(num_clicks + (*position == 0) as usize)
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        ),
        6
    );
}
