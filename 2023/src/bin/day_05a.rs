fn solve(input: &str) -> u64 {
    let mut input = input.trim().split("\n\n");
    let seeds = input.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let mut seeds: Vec<_> = seeds
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    input
        .map(|l| {
            l.lines()
                .skip(1)
                .map(|mapper| {
                    let mut mapper = mapper.split_whitespace().map(|n| n.parse::<u64>().unwrap());
                    let destination_range = mapper.next().unwrap();
                    let source_range = mapper.next().unwrap();
                    let range_length = mapper.next().unwrap();
                    (
                        source_range..(source_range + range_length),
                        destination_range,
                    )
                })
                .collect::<Vec<_>>()
        })
        .for_each(|mapper| {
            seeds.iter_mut().for_each(|s| {
                if let Some(next) = mapper.iter().find_map(|(mapper, destination)| {
                    if mapper.contains(s) {
                        Some(destination + *s - mapper.start)
                    } else {
                        None
                    }
                }) {
                    *s = next;
                }
            })
        });

    seeds.into_iter().min().unwrap()
}

common::read_main!();

#[test]
fn example() {
    let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    assert_eq!(solve(input), 35);
}
