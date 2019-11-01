use std::collections::HashSet;

fn solve(input: &str) -> usize {
    struct State {
        previous: [char; 2],
        rest: HashSet<[char; 2]>,
    }

    let is_nice = |l: &&str| {
        let l = l.chars().collect::<Vec<_>>();
        let sandwich = l.windows(3).any(|w| w[0] == w[2]);
        let state = State {
            previous: [l[0], l[1]],
            rest: HashSet::new(),
        };
        let pair_repeated = l
            .windows(2)
            .skip(1)
            .scan(state, |state, pair| {
                let pair = [pair[0], pair[1]];
                let repeated = state.rest.contains(&pair);
                state.rest.insert(state.previous);
                state.previous = pair;
                Some(repeated)
            })
            .any(|repeated| repeated);
        sandwich && pair_repeated
    };

    input.lines().filter(is_nice).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singles() {
        assert_eq!(solve("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(solve("xxyxx"), 1);
        assert_eq!(solve("uurcxstgmygtbstg"), 0);
        assert_eq!(solve("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn test_many() {
        let input = r"qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";
        assert_eq!(solve(input), 2);
    }
}

common::read_main!();
