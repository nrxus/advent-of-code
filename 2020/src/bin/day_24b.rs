use std::collections::HashSet;

fn solve(tiles: &str) -> usize {
    let mut black_tiles = HashSet::new();

    let east = |p: (i16, i16)| (p.0 + 2, p.1 + 1);
    let southeast = |p: (i16, i16)| (p.0 + 1, p.1 - 1);
    let southwest = |p: (i16, i16)| (p.0 - 1, p.1 - 2);
    let west = |p: (i16, i16)| (p.0 - 2, p.1 - 1);
    let northwest = |p: (i16, i16)| (p.0 - 1, p.1 + 1);
    let northeast = |p: (i16, i16)| (p.0 + 1, p.1 + 2);

    let neighbors = |p: (i16, i16)| {
        vec![
            east(p),
            southeast(p),
            southwest(p),
            west(p),
            northwest(p),
            northeast(p),
        ]
    };

    tiles
        .trim()
        .lines()
        .map(|t| {
            let mut position = (0_i16, 0_i16);
            let mut start = 0;
            let mut end = 0;

            while end < t.len() {
                end += 1;
                debug_assert!(end - start < 3);
                position = match &t[start..end] {
                    "e" => east(position),
                    "se" => southeast(position),
                    "sw" => southwest(position),
                    "w" => west(position),
                    "nw" => northwest(position),
                    "ne" => northeast(position),
                    "n" | "s" => continue,
                    unexpected => panic!("unexpected: {:?}", unexpected),
                };
                start = end;
            }

            position
        })
        .for_each(|position| {
            if black_tiles.contains(&position) {
                black_tiles.remove(&position);
            } else {
                black_tiles.insert(position);
            }
        });

    (0..100).for_each(|_| {
        let mut new_tiles: HashSet<_> = black_tiles
            .iter()
            .flat_map(|&black_tile| {
                neighbors(black_tile)
                    .into_iter()
                    .chain(std::iter::once(black_tile))
            })
            .collect();

        new_tiles.retain(|t| {
            let black_neighbors = neighbors(*t)
                .into_iter()
                .filter(|n| black_tiles.contains(n))
                .take(3)
                .count();
            black_neighbors == 2 || (black_tiles.contains(&*t) && black_neighbors == 1)
        });

        black_tiles = new_tiles;
    });

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(solve(input), 2208);
    }
}

common::read_main!();
