use std::{
    collections::{BinaryHeap, HashMap},
    ops::RangeInclusive,
};

fn solve(input: &str) -> usize {
    let mut verticals: BinaryHeap<_> = input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let direction = line.next().unwrap();
            let length: u8 = line.next().unwrap().parse().unwrap();
            (direction, length)
        })
        .scan((0_i64, 0_i64), |(x, y), (direction, length)| {
            let (past_x, past_y) = (*x, *y);
            let line = match direction {
                "R" => {
                    *x += length as i64;
                    Orientation::Horizontal(Horizontal {
                        x: past_x..=*x,
                        y: past_y,
                    })
                }
                "L" => {
                    *x -= length as i64;
                    Orientation::Horizontal(Horizontal {
                        x: *x..=past_x,
                        y: past_y,
                    })
                }
                "U" => {
                    *y -= length as i64;
                    Orientation::Vertical(Vertical {
                        x: past_x,
                        y: *y..=past_y,
                    })
                }
                "D" => {
                    *y += length as i64;
                    Orientation::Vertical(Vertical {
                        x: past_x,
                        y: past_y..=*y,
                    })
                }
                _ => unreachable!(),
            };
            Some(line)
        })
        .filter_map(|line| match line {
            Orientation::Horizontal(_) => None,
            Orientation::Vertical(v) => Some(v),
        })
        .collect();

    let mut horizontals = HashMap::new();
    let mut sum = 0;
    while let Some(mut right) = verticals.pop() {
        let mut extend = vec![];

        while let Some(mut left) = verticals.pop() {
            if left.y.end() <= right.y.start() || right.y.end() <= left.y.start() {
                extend.push(left);
                continue;
            }

            if left.y.end() < right.y.end() {
                extend.push(Vertical {
                    x: right.x,
                    y: (*left.y.end())..=*right.y.end(),
                });
                right.y = *right.y.start()..=*left.y.end();
            }

            if right.y.end() < left.y.end() {
                extend.push(Vertical {
                    x: left.x,
                    y: (*right.y.end())..=*left.y.end(),
                });
                left.y = *left.y.start()..=*right.y.end();
            }

            if right.y.start() < left.y.start() {
                extend.push(Vertical {
                    x: right.x,
                    y: *right.y.start()..=*left.y.start(),
                });
                right.y = *left.y.start()..=*right.y.end();
            }

            if left.y.start() < right.y.start() {
                extend.push(Vertical {
                    x: left.x,
                    y: *left.y.start()..=*right.y.start(),
                });
                left.y = *right.y.start()..=*left.y.end();
            }

            let horizontal = left.x..=right.x;
            horizontals
                .entry(*left.y.start())
                .or_insert(vec![])
                .push(horizontal.clone());
            horizontals
                .entry(*left.y.end())
                .or_insert(vec![])
                .push(horizontal);

            sum += (right.x - left.x + 1) as usize * (left.y.end() - left.y.start() + 1) as usize;
            break;
        }

        verticals.extend(extend);
    }

    let num_overlaps: usize = horizontals
        .into_iter()
        .map(|(_, mut lines)| {
            let mut num_overlaps: usize = 0;
            while let Some(mut line) = lines.pop() {
                num_overlaps += lines
                    .iter()
                    .map(|other| {
                        if line.end() <= other.start() || other.end() <= line.start() {
                            return 0;
                        }

                        if other.end() < line.end() {
                            line = *line.start()..=*other.end();
                        }

                        if line.start() < other.start() {
                            line = *other.start()..=*line.end();
                        }

                        (*line.end() - *line.start() + 1) as usize
                    })
                    .sum::<usize>();
            }
            num_overlaps
        })
        .sum();

    sum - num_overlaps
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Horizontal {
    x: RangeInclusive<i64>,
    y: i64,
}

impl PartialOrd for Horizontal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Horizontal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vertical {
    x: i64,
    y: RangeInclusive<i64>,
}

impl PartialOrd for Vertical {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Vertical {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

common::read_main!();

#[test]
fn example() {
    let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    assert_eq!(solve(input), 62);
}
