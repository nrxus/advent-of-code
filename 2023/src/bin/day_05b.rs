use std::ops::Range;

fn solve(input: &str) -> u64 {
    let mut input = input.trim().split("\n\n");
    let seeds = input.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<_> = seeds
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let seeds: Vec<_> = seeds
        .chunks_exact(2)
        .map(|pair| (pair[0]..(pair[0] + pair[1])))
        .collect();

    let out = input
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
        .fold(seeds, |sources, mapper| source_to_dest(sources, mapper));

    out.into_iter().map(|r| r.start).min().unwrap()
}

fn source_to_dest(
    mut sources: Vec<Range<u64>>,
    mut mapper: Vec<(Range<u64>, u64)>,
) -> Vec<Range<u64>> {
    sources.sort_by_key(|s| s.start);
    mapper.sort_by_key(|(r, _)| r.start);

    let mut out = Vec::with_capacity(sources.len());

    while let Some(mut source) = sources.pop() {
        if let Some((target, destination)) = mapper.pop() {
            if source.end <= target.start {
                // target range is too high -- keep searching
                sources.push(source);
                continue;
            }

            if target.end <= source.start {
                // target is too low -- copy as-is and re-use range
                out.push(source);
                mapper.push((target, destination));
                continue;
            }

            if target.end < source.end {
                // a portion of the end is too high -- copy it and trim
                out.push(target.end..source.end);
                source.end = target.end;
            }

            if source.start < target.start {
                // a portion of the start is too low -- put it back on the search for later and trim
                sources.push(source.start..target.start);
                source.start = target.start;
            }

            if target.start < source.start {
                // a portion of the map can be re-used
                mapper.push((target.start..source.start, destination));
            }

            let length = source.end - source.start;
            let out_start = destination + source.start - target.start;
            out.push(out_start..(out_start + length));
        } else {
            out.push(source);
        }
    }

    // TODO: do I need to merge overlaps?? I guess not??
    out
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
    assert_eq!(solve(input), 46);
}
