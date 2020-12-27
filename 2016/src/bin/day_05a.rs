fn solve(door_id: &str) -> String {
    let door_id = door_id.trim();
    (0..)
        .map(|index| md5::compute(format!("{}{}", door_id, index)))
        .filter_map(|digest| {
            if digest[0..2] == [0, 0] {
                std::char::from_digit(digest[2] as u32, 16)
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"abc
";
        assert_eq!(solve(input), "18f47a30");
    }
}

common::read_main!();
