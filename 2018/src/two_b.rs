use std::collections::HashSet;

fn solve(input: &str) -> String {
    let mut previous: HashSet<&str> = HashSet::new();

    for id in input.lines() {
        match previous.iter().find(|old_id| {
            old_id
                .chars()
                .zip(id.chars())
                .filter(|(a, b)| a != b)
                .take(2)
                .count()
                == 1
        }) {
            Some(old_id) => {
                return old_id
                    .chars()
                    .zip(id.chars())
                    .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                    .collect();
            }
            None => {
                previous.insert(id);
            }
        }
    }

    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;
        assert_eq!(solve(input), "fgij".to_string())
    }
}

common::read_main!();
