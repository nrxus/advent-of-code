use std::ops::RangeInclusive;

fn solve(input: &str) -> usize {
    let target_area = input.trim().strip_prefix("target area: ").unwrap();
    let (x, y) = target_area.split_once(", ").unwrap();
    let x_target = parse_range(x.strip_prefix("x=").unwrap()).unwrap();
    let y_target = parse_range(y.strip_prefix("y=").unwrap()).unwrap();

    let min_x = estimate_start_triangle(*x_target.start()).ceil() as u16;
    let max_x = *x_target.end() as u16;

    let min_y = *y_target.start();
    let max_y = y_target.start().abs() - 1;

    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .filter(|&(mut speed)| {
            let (mut x, mut y) = (0_u16, 0_i16);

            while (x as i16) <= *x_target.end() && y >= *y_target.start() {
                x += speed.0;
                y += speed.1;

                speed.0 = speed.0.saturating_sub(1);
                speed.1 -= 1;

                if x_target.contains(&(x as i16)) && y_target.contains(&y) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn estimate_start_triangle(goal: i16) -> f64 {
    (((1 + 8 * goal) as f64).sqrt() - 1_f64) / 2_f64
}

fn parse_range(s: &str) -> Result<RangeInclusive<i16>, Box<dyn std::error::Error>> {
    let (min, max) = s.split_once("..").ok_or_else(|| "missing '..".to_owned())?;
    Ok(min.parse()?..=max.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"target area: x=20..30, y=-10..-5";
        assert_eq!(solve(input), 112);
    }
}

common::read_main!();
