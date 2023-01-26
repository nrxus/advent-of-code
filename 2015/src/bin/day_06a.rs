use common::read_main;

fn solve(input: &str) -> usize {
    // could be bitvec
    let mut lit = vec![false; 1000 * 1000];

    input.trim().lines().for_each(|line| {
        let mut line = line.split_whitespace();
        let instruction = match line.next().unwrap() {
            "turn" => match line.next().unwrap() {
                "on" => Instruction::TurnOn,
                "off" => Instruction::TurnOff,
                unknown => panic!("unexpeced instruction: turn {unknown}"),
            },
            "toggle" => Instruction::Toggle,
            unknown => panic!("unexpeced instruction: {unknown}"),
        };

        let start: (u16, u16) = {
            let point = line.next().unwrap();
            let (x, y) = point.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        };

        let _through = line.next().unwrap();

        let end: (u16, u16) = {
            let point = line.next().unwrap();
            let (x, y) = point.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        };

        let range = (start.0..=end.0)
            .flat_map(move |x| (start.1..=end.1).map(move |y| x as usize + y as usize * 1000));

        match instruction {
            Instruction::TurnOn => range.for_each(|p| {
                lit[p] = true;
            }),
            Instruction::TurnOff => range.for_each(|p| {
                lit[p] = false;
            }),
            Instruction::Toggle => range.for_each(|p| {
                lit[p] = !lit[p];
            }),
        }
    });

    lit.into_iter().filter(|l| *l).count()
}

enum Instruction {
    TurnOff,
    TurnOn,
    Toggle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
        assert_eq!(solve(input), 998996);
    }
}

read_main!();
