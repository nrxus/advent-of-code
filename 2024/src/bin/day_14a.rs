fn solve(input: &str) -> usize {
    let mut quadrants = [0, 0, 0, 0];

    input
        .trim()
        .lines()
        .map(|robot| {
            let (p, v) = robot.split_once(' ').unwrap();
            let p = p.strip_prefix("p=").unwrap();
            let (x, y) = p.split_once(',').unwrap();
            let mut p = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

            let v = v.strip_prefix("v=").unwrap();
            let (x, y) = v.split_once(',').unwrap();
            let v = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

            p.0 = (p.0 + v.0 * 100).rem_euclid(SIZE.0);
            p.1 = (p.1 + v.1 * 100).rem_euclid(SIZE.1);

            p
        })
        .for_each(|(x, y)| {
            let mut pos = if x < SIZE.0 / 2 {
                0
            } else if x > SIZE.0 / 2 {
                1
            } else {
                return;
            };
            pos += if y < SIZE.1 / 2 {
                0
            } else if y > SIZE.1 / 2 {
                2
            } else {
                return;
            };
            quadrants[pos] += 1;
        });

    quadrants.into_iter().product()
}

#[cfg(test)]
const SIZE: (i32, i32) = (11, 7);

#[cfg(not(test))]
const SIZE: (i32, i32) = (101, 103);

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        ),
        12
    );
}
