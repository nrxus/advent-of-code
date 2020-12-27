fn solve(door_id: &str) -> String {
    let door_id = door_id.trim();
    let mut password = vec![None, None, None, None, None, None, None, None];

    for digest in (0..).map(|index| md5::compute(format!("{}{}", door_id, index))) {
        if digest[0..2] != [0, 0] {
            continue
        }

        let position = digest[2] as usize;

        if position >= password.len() || password[position].is_some() {
            continue
        }

        password[position] = std::char::from_digit((digest[3] >> 4) as u32, 16);

        if password.iter().all(Option::is_some) {
            break;
        }
    }

    password.into_iter().map(|c| c.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"abc
";
        assert_eq!(solve(input), "05ace8e3");
    }
}

common::read_main!();
