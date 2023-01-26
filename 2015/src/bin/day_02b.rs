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
            let volume = l * w * h;
            let min_perimeter = 2 * (l + h).min(l + w).min(w + h);

            min_perimeter + volume
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
        assert_eq!(solve(input), 34 + 14);
    }
}

read_main!();
