fn solve(input: &str) -> usize {
    length(input.trim(), 40)
}

fn length(input: &str, iterations: usize) -> usize {
    (0..iterations)
        .fold(input.to_string(), |s, _| look_and_say(s.as_str()))
        .len()
}

fn look_and_say(input: &str) -> String {
    input
        .chars()
        .fold(vec![], |mut acc, c| {
            if let Some((ch, count)) = acc.pop() {
                if ch == c {
                    acc.push((ch, count + 1));
                } else {
                    acc.push((ch, count));
                    acc.push((c, 1));
                }
            } else {
                acc.push((c, 1));
            }
            acc
        })
        .into_iter()
        .flat_map(|(ch, count)| vec![std::char::from_digit(count, 10).unwrap(), ch])
        .collect()
}

#[cfg(test)]
mod ten_a {
    use super::*;

    #[test]
    fn more_complex() {
        assert_eq!(look_and_say("1321131112"), "11131221133112");
        assert_eq!(look_and_say("11131221133112"), "3113112221232112");
        assert_eq!(look_and_say("3113112221232112"), "1321132132111213122112");
    }

    #[test]
    fn iteration() {
        let solution = (0..5).fold("1".to_string(), |s, _| look_and_say(s.as_str()));
        assert_eq!(solution, "312211");
    }

    #[test]
    fn solution_length() {
        assert_eq!(length("1", 5), 6);
        assert_eq!(length("1321131112", 3), 22);
    }
}

common::read_main!();
