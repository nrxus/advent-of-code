fn solve(input: &str) -> u32 {
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    const MAXES: [u32; 3] = [12, 13, 14];

    input
        .trim()
        .lines()
        .filter_map(|game| {
            let mut game_maxes = [0, 0, 0];

            let (id, game) = game.split_once(':').unwrap();
            game.split(';')
                .flat_map(|s| s.split(','))
                .for_each(|cubes| {
                    let (num, rest) = cubes.trim().split_once(' ').unwrap();
                    let num: u32 = num.parse().unwrap();

                    for (i, c) in COLORS.into_iter().enumerate() {
                        if rest.contains(c) {
                            game_maxes[i] = game_maxes[i].max(num);
                        }
                    }
                });

            let possible = MAXES
                .into_iter()
                .zip(game_maxes)
                .all(|(max, game_max)| max >= game_max);

            if possible {
                Some(id)
            } else {
                None
            }
        })
        .map(|id| {
            let (_, id) = id.split_once(' ').unwrap();
            id.parse::<u32>().unwrap()
        })
        .sum()
}

common::read_main!();

#[test]
fn example() {
    let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(solve(input), 8);
}
