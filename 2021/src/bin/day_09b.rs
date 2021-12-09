use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let input = input.trim();

    let width = input.lines().next().map(|l| l.len()).expect("empty input");
    let heightmap: Vec<u32> = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let find_neighbors = |index: usize| {
        let left = index
            .checked_sub(1)
            .filter(|left| left % width < index % width);

        let right = Some(index + 1).filter(|right| right % width > index % width);

        let up = Some(index + width).filter(|up| *up < heightmap.len());

        let down = index.checked_sub(width);

        [left, right, up, down]
    };

    let lows = heightmap
        .iter()
        .enumerate()
        .filter(|&(i, depth)| {
            find_neighbors(i)
                .into_iter()
                .flatten()
                .map(|n| heightmap[n])
                .all(|n| n > *depth)
        })
        .map(|(index, _)| index);

    let mut basin_sizes: Vec<usize> = lows
        .map(|low| {
            let mut explored = HashSet::new();
            let mut frontier = vec![low];

            while let Some(depth_index) = frontier.pop() {
                if explored.contains(&depth_index) {
                    continue;
                }

                if heightmap[depth_index] == 9 {
                    continue;
                }

                explored.insert(depth_index);

                frontier.extend(find_neighbors(depth_index).into_iter().flatten())
            }

            explored.len()
        })
        .collect();

    basin_sizes.sort_by(|a, b| b.cmp(a));

    basin_sizes.into_iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(solve(input), 1134);
    }
}

common::read_main!();
