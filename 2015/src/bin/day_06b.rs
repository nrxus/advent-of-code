use common::read_main;

fn solve(input: &str) -> u32 {
    let mut lit = vec![0_u32; 1000 * 1000];

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
                lit[p] += 1;
            }),
            Instruction::TurnOff => range.for_each(|p| {
                lit[p] = lit[p].saturating_sub(1);
            }),
            Instruction::Toggle => range.for_each(|p| {
                lit[p] += 2;
            }),
        }
    });

    lit.into_iter().sum()
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
        let input = r"turn on 0,0 through 0,0
toggle 0,0 through 999,999";
        assert_eq!(solve(input), 2000001);
    }
}

read_main!();
