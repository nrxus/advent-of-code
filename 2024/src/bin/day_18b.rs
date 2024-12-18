use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> String {
    let input = input.trim().lines().map(|coords| {
        let (x, y) = coords.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    });

    let start = (0, 0);
    let end = (BOUNDARY, BOUNDARY);
    let mut corrupted: HashSet<(usize, usize)> = HashSet::new();

    'outer: for c in input {
        let mut frontier = VecDeque::from_iter([(start, 0)]);
        let mut explored = HashSet::new();
        corrupted.insert(c);

        while let Some((pos, steps)) = frontier.pop_front() {
            if pos == end {
                continue 'outer;
            }
            let (x, y) = pos;
            let left = x.checked_sub(1);
            let right = Some(x + 1).filter(|x| *x <= BOUNDARY);
            let up = y.checked_sub(1);
            let down = Some(y + 1).filter(|y| *y <= BOUNDARY);
            let neighbors = [
                up.map(|y| (x, y)),
                down.map(|y| (x, y)),
                right.map(|x| (x, y)),
                left.map(|x| (x, y)),
            ]
            .into_iter()
            .flatten()
            .filter(|pos| !corrupted.contains(pos))
            .filter(|pos| explored.insert(*pos))
            .map(|pos| (pos, steps + 1));

            frontier.extend(neighbors)
        }

        return format!("{},{}", c.0, c.1);
    }

    panic!("no byte blocked exit")
}

#[cfg(test)]
const BOUNDARY: usize = 6;

#[cfg(not(test))]
const BOUNDARY: usize = 70;

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
        ),
        "6,1"
    );
}
