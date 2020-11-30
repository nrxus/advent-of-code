use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let origin: (i32, i32) = (0, 0);

    let wires: Vec<_> = input
        .trim()
        .lines()
        .map(|path| {
            let all_coords = path
                .split(",")
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
                .enumerate();

            // do this instead of collect to handle collisions correctly
            let mut coords_with_timings: HashMap<(i32, i32), usize> =
                HashMap::with_capacity(all_coords.size_hint().0);
            all_coords.for_each(|(k, v)| {
                coords_with_timings.entry(v).or_insert(k);
            });
            coords_with_timings
        })
        .collect();

    let first_wire_path = wires[0].keys().collect::<HashSet<_>>();
    let second_wire_path = wires[1].keys().collect::<HashSet<_>>();

    first_wire_path
        .intersection(&second_wire_path)
        .map(|coord| wires[0][coord] + wires[1][coord])
        .min()
        .unwrap()
        + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(solve(input), 30);

        let input = r"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve(input), 610);

        let input = r"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve(input), 410);
    }
}

common::read_main!();
