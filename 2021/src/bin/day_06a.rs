fn solve(input: &str) -> usize {
    let mut days = [0_usize; 9];

    input
        .trim()
        .split(',')
        .try_for_each(|day| {
            let day: usize = day.parse()?;
            days[day] += 1;
            Result::<(), Box<dyn std::error::Error>>::Ok(())
        })
        .unwrap();

    for day in (0..80).map(|day| day % 9) {
        let next_self = (day + 7) % 9;
        days[next_self] += days[day];
    }

    days.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"3,4,3,1,2";
        assert_eq!(solve(input), 5934);
    }
}

common::read_main!();
