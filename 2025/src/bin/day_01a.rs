fn solve(input: &str) -> usize {
    const START_POS: u16 = 50;

    input
        .trim()
        .lines()
        .map(|instruction| {
            // any rotation past 100 can be simplified with just the remainder
            let number: u16 = instruction[1..]
                .parse::<u16>()
                .expect("expected number of clicks")
                % 100;
            let direction = &instruction[0..1];
            (number, direction)
        })
        .scan(START_POS, |position, (number, direction)| {
            match direction {
                "L" => {
                    let (p, underflow) = position.overflowing_sub(number);
                    *position = if underflow { p.wrapping_add(100) } else { p };
                }
                "R" => {
                    *position = (*position + number) % 100;
                }
                _ => {
                    panic!("expected L or R")
                }
            }
            Some(*position)
        })
        .filter(|pos| *pos == 0)
        .count()
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
        3
    );
}
