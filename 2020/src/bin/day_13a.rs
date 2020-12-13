fn solve(notes: &str) -> u32 {
    let mut notes = notes.trim().lines();
    let earliest: u32 = notes.next().unwrap().parse().unwrap();
    let buses = notes.next().unwrap();

    let (id, minutes) = buses
        .split(',')
        .filter(|b| *b != "x")
        .map(|b| b.parse::<u32>().unwrap())
        .map(|id| (id, ((earliest / id) + 1) * id))
        .min_by_key(|(_, time)| *time)
        .unwrap();

    id * (minutes - earliest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"939
7,13,x,x,59,x,31,19";
        assert_eq!(solve(input), 295);
    }
}

common::read_main!();
