fn solve(input: &str) -> i32 {
    let target_area = input.trim().strip_prefix("target area: ").unwrap();
    let (_, y) = target_area.split_once(", ").unwrap();
    let (y_min, _) = y.strip_prefix("y=").unwrap().split_once("..").unwrap();
    let y_min: i32 = y_min.parse().unwrap();

    // I am 90% sure this only works if y not above start_point
    assert!(y_min < 1);

    (y_min) * (y_min + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"target area: x=20..30, y=-10..-5";
        assert_eq!(solve(input), 45);
    }
}

common::read_main!();
