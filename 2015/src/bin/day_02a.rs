use common::read_main;

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.split('x').map(|n| n.parse::<u32>().unwrap());
            let (l, w, h) = (
                line.next().unwrap(),
                line.next().unwrap(),
                line.next().unwrap(),
            );

            let bottom = l * w;
            let side = l * h;
            let front = w * h;
            2 * (bottom + side + front) + bottom.min(side).min(front)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"2x3x4
1x1x10";
        assert_eq!(solve(input), 58 + 43);
    }
}

read_main!();
