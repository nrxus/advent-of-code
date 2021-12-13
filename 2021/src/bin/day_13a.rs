use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let (points, folds) = input.trim().split_once("\n\n").unwrap();
    let points: HashSet<_> = points
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            let x: u16 = x.parse().unwrap();
            let y: u16 = y.parse().unwrap();

            (x, y)
        })
        .collect();

    // we only care about the first fold
    let fold = folds.lines().next().unwrap();
    let fold = fold.strip_prefix("fold along ").unwrap();
    let (fold_direction, fold_magnitude) = fold.split_once('=').unwrap();
    let fold_magnitude: u16 = fold_magnitude.parse().unwrap();

    let points: HashSet<_> = match fold_direction {
        "x" => points
            .into_iter()
            .map(|(x, y)| {
                let x = match x.cmp(&fold_magnitude) {
                    std::cmp::Ordering::Less => x,
                    std::cmp::Ordering::Equal => panic!("folding on line with point"),
                    std::cmp::Ordering::Greater => fold_magnitude - (x - fold_magnitude),
                };

                (x, y)
            })
            .collect(),
        "y" => points
            .into_iter()
            .map(|(x, y)| {
                let y = match y.cmp(&fold_magnitude) {
                    std::cmp::Ordering::Less => y,
                    std::cmp::Ordering::Equal => panic!("folding on line with point"),
                    std::cmp::Ordering::Greater => fold_magnitude - (y - fold_magnitude),
                };

                (x, y)
            })
            .collect(),
        e => panic!("unexpected fold direction: {}", e),
    };

    points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(solve(input), 17);
    }
}

common::read_main!();
