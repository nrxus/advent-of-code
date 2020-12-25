use std::collections::HashSet;

fn solve(tiles: &str) -> usize {
    let mut white_tiles = HashSet::new();

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
                match &t[start..end] {
                    "e" => {
                        position.0 += 2;
                        position.1 += 1;
                    }
                    "se" => {
                        position.0 += 1;
                        position.1 -= 1;
                    }
                    "sw" => {
                        position.0 -= 1;
                        position.1 -= 2;
                    }
                    "w" => {
                        position.0 -= 2;
                        position.1 -= 1;
                    }
                    "nw" => {
                        position.0 -= 1;
                        position.1 += 1;
                    }
                    "ne" => {
                        position.0 += 1;
                        position.1 += 2;
                    }
                    "n" | "s" => continue,
                    unexpected => panic!("unexpected: {:?}", unexpected),
                }
                start = end;
            }

            if white_tiles.contains(&position) {
                white_tiles.remove(&position)
            } else {
                white_tiles.insert(position)
            }
        })
        .for_each(|_| {});

    white_tiles.len()
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
        assert_eq!(solve(input), 10);
    }
}

common::read_main!();
