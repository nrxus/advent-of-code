fn solve(notes: &str) -> u64 {
    let mut constraints = notes
        .trim()
        .lines()
        .last()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| {
            if id == "x" {
                None
            } else {
                Some((i, id.parse().unwrap()))
            }
        })
        .map(|(i, id)| Constraint {
            remainder: (id - (i as u64 % id)) % id,
            modulus: id,
        });

    let first = constraints.next().unwrap();

    constraints
        .fold(first, |acc, constraint| {
            let mut remainder = acc.remainder;
            while remainder % constraint.modulus != constraint.remainder {
                remainder += acc.modulus;
            }

            Constraint {
                remainder,
                modulus: acc.modulus * constraint.modulus,
            }
        })
        .remainder
}

#[derive(Debug)]
struct Constraint {
    remainder: u64,
    modulus: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"939
7,13,x,x,59,x,31,19";
        assert_eq!(solve(input), 1068781);
    }

    #[test]
    fn example_two() {
        let input = r"939
17,x,13,19";
        assert_eq!(solve(input), 3417);
    }

    #[test]
    fn example_three() {
        let input = r"939
67,7,59,61";
        assert_eq!(solve(input), 754018);
    }

    #[test]
    fn example_four() {
        let input = r"939
67,x,7,59,61";
        assert_eq!(solve(input), 779210);
    }

    #[test]
    fn example_five() {
        let input = r"939
67,7,x,59,61";
        assert_eq!(solve(input), 1261476);
    }

    #[test]
    fn example_six() {
        let input = r"939
1789,37,47,1889";
        assert_eq!(solve(input), 1202161486);
    }
}

common::read_main!();
