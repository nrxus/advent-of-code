use std::collections::HashSet;

fn solve(initial: &str) -> usize {
    let mut actives: HashSet<(i32, i32, i32, i32)> = initial
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some((x as i32, y as i32, 0, 0)),
                e => panic!("unexpected: {:?}", e),
            })
        })
        .collect();

    let mut min = (0_i32, 0_i32, 0_i32, 0_i32);
    let mut max = (0, 0, 0, 0);

    max.0 = actives.iter().map(|p| p.0).max().unwrap();
    max.1 = actives.iter().map(|p| p.1).max().unwrap();
    min.0 = actives.iter().map(|p| p.0).min().unwrap();
    min.1 = actives.iter().map(|p| p.1).min().unwrap();

    (0..6).for_each(|_| {
        actives = (min.0 - 1..=max.0 + 1)
            .flat_map(|x| (min.1 - 1..=max.1 + 1).map(move |y| (x, y)))
            .flat_map(|(x, y)| (0_i32..=max.2 + 1).map(move |z| (x, y, z)))
            .flat_map(|(x, y, z)| (0_i32..=max.3 + 1).map(move |w| (x, y, z, w)))
            .filter_map(|(x, y, z, w)| {
                let active_count = neighbors((x, y, z.abs(), w.abs()))
                    .filter(|p| actives.contains(p))
                    .take(4)
                    .count();
                if active_count == 3 || (actives.contains(&(x, y, z, w)) && active_count == 2) {
                    Some((x, y, z, w))
                } else {
                    None
                }
            })
            .collect();

        max.0 = actives.iter().map(|p| p.0).max().unwrap();
        max.1 = actives.iter().map(|p| p.1).max().unwrap();
        max.2 = actives.iter().map(|p| p.2).max().unwrap();
        max.3 = actives.iter().map(|p| p.3).max().unwrap();
        min.0 = actives.iter().map(|p| p.0).min().unwrap();
        min.1 = actives.iter().map(|p| p.1).min().unwrap();
    });

    let not_mirrored = actives.iter().filter(|(_,_,z,w)| *z == 0 && *w == 0).count();
    let mirrored_once = actives.iter().filter(|(_,_,z,w)| (*z == 0) ^ (*w == 0)).count();
    actives.len() * 4 - 3 * not_mirrored - 2 * mirrored_once
}

fn neighbors((x, y, z, w): (i32, i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    (x - 1..=x + 1)
        .flat_map(move |x| (y - 1..=y + 1).map(move |y| (x, y)))
        .flat_map(move |(x, y)| (z - 1..=z + 1).map(move |z| (x, y, z.abs())))
        .flat_map(move |(x, y, z)| (w - 1..=w + 1).map(move |w| (x, y, z, w.abs())))
        .filter(move |p| *p != (x, y, z, w))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r".#.
..#
###";
        assert_eq!(solve(input), 848);
    }
}

common::read_main!();
