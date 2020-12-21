use std::collections::HashMap;

fn solve(images: &str) -> u64 {
    let images = images.trim();
    let images: HashMap<_, _> = images
        .split("\n\n")
        .map(|image| {
            let mut image = image.splitn(2, '\n');
            let id = image.next().unwrap();
            let id: u64 = id["Tile ".len()..id.len() - 1].parse().unwrap();
            let image = image.next().unwrap();
            let mut lines = image.lines();
            (
                id,
                Image {
                    top: lines.next().unwrap(),
                    bottom: lines.last().unwrap(),
                    left: image.lines().map(|l| l.chars().next().unwrap()).collect(),
                    right: image.lines().map(|l| l.chars().last().unwrap()).collect(),
                },
            )
        })
        .collect();

    let mut corners = vec![];
    for (idx, image) in images.iter() {
        let matched_edges = [
            image.top,
            image.bottom,
            image.left.as_str(),
            image.right.as_str(),
        ]
        .iter()
        .filter(|&&edge| {
            images
                .iter()
                .filter(|(idx2, _)| *idx2 != idx)
                .any(|(_, image2)| {
                    edge == image2.top
                        || edge == image2.bottom
                        || edge == image2.left.as_str()
                        || edge == image2.right.as_str()
                        || edge.chars().eq(image2.top.chars().rev())
                        || edge.chars().eq(image2.bottom.chars().rev())
                        || edge.chars().eq(image2.left.chars().rev())
                        || edge.chars().eq(image2.right.chars().rev())
                })
        })
        .count();

        debug_assert!(matched_edges >= 2);
        if matched_edges == 2 {
            corners.push(*idx);
        }
    }

    debug_assert_eq!(corners.len(), 4);
    corners.into_iter().product()
}

struct Image<'s> {
    top: &'s str,
    bottom: &'s str,
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(solve(input), 20899048083289);
    }
}

common::read_main!();
