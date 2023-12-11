use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let input = input.trim();

    let mut galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| ((x, y)))
        })
        .collect();

    let occupied_xs: HashSet<_> = galaxies.iter().map(|(x, _)| *x).collect();
    let occupied_ys: HashSet<_> = galaxies.iter().map(|(_, y)| *y).collect();

    let mut sum = 0;
    while let Some(galaxy) = galaxies.pop() {
        sum += galaxies
            .iter()
            .map(|other| {
                let min_x = galaxy.0.min(other.0);
                let max_x = galaxy.0.max(other.0);
                let min_y = galaxy.1.min(other.1);
                let max_y = galaxy.1.max(other.1);

                let mut sum = (max_x - min_x) + (max_y - min_y);

                for x in min_x..max_x {
                    if !occupied_xs.contains(&x) {
                        sum += 1
                    }
                }

                for y in min_y..max_y {
                    if !occupied_ys.contains(&y) {
                        sum += 1
                    }
                }

                sum
            })
            .sum::<usize>();
    }

    sum
}

common::read_main!();

#[test]
fn example_one() {
    let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(solve(input), 374);
}
