fn solve(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    input.sort_unstable();

    let average = input.iter().sum::<u32>() / input.len() as u32;

    // idk one of them??
    let sum_using_floor = sum_triangle_distance(input.iter(), average);
    let sum_using_ceil = sum_triangle_distance(input.iter(), average + 1);

    sum_using_floor.min(sum_using_ceil)
}

fn sum_triangle_distance<'a>(numbers: impl Iterator<Item = &'a u32>, target: u32) -> u32 {
    numbers
        .map(|&d| {
            let distance = if d > target { d - target } else { target - d };
            distance * (distance + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve(input), 168);
    }

    #[test]
    fn example_2() {
        let input = r"1,2,4";
        assert_eq!(solve(input), 4);
    }
}

common::read_main!();
