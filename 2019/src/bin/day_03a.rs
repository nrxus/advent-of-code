use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    let origin: (i32, i32) = (0, 0);

    let wires: Vec<_> = input
        .trim()
        .lines()
        .map(|path| {
            path.split(",")
                .scan(origin, |position, coords: &str| {
                    let distance: u32 = coords[1..].parse().unwrap();
                    let path = match &coords[0..1] {
                        "U" => (1..=distance)
                            .map(|d| (position.0, position.1 + d as i32))
                            .collect::<Vec<_>>(),
                        "R" => (1..=distance)
                            .map(|d| (position.0 + d as i32, position.1))
                            .collect::<Vec<_>>(),
                        "D" => (1..=distance)
                            .map(|d| (position.0, position.1 - d as i32))
                            .collect::<Vec<_>>(),
                        "L" => (1..=distance)
                            .map(|d| (position.0 - d as i32, position.1))
                            .collect::<Vec<_>>(),
                        _ => unreachable!(),
                    };
                    *position = *(path.last().unwrap());
                    Some(path)
                })
                .flat_map(|path| path)
                .collect::<HashSet<_>>()
        })
        .collect();
    wires[0]
        .intersection(&wires[1])
        .map(|(x, y)| (x.abs() + y.abs()) as u32)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(solve(input), 6);

        let input = r"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve(input), 159);

        let input = r"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve(input), 135);
    }
}

common::read_main!();
