use regex::Regex;
use std::collections::HashMap;

fn solve(program: &str) -> u64 {
    let mask_regex = Regex::new(r"mask = (?P<mask>.*)").unwrap();
    let mem_regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();

    let mut mask: HashMap<usize, char> = HashMap::new();
    let mut memory: HashMap<u16, u64> = HashMap::new();
    for instruction in program.trim().lines() {
        if let Some(captures) = mask_regex.captures(instruction) {
            mask = captures
                .name("mask")
                .unwrap()
                .as_str()
                .char_indices()
                .filter_map(|(i, c)| if c == 'X' { None } else { Some((i, c)) })
                .collect();
            continue;
        }
        let captures = mem_regex.captures(instruction).unwrap();
        let address: u16 = captures.name("address").unwrap().as_str().parse().unwrap();
        let value: u64 = captures.name("value").unwrap().as_str().parse().unwrap();
        let mut value = format!("{:036b}", value);
        mask.iter()
            .for_each(|(&i, &b)| unsafe { value.as_mut_vec()[i] = b as u8 });
        memory.insert(address, u64::from_str_radix(&value, 2).unwrap());
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(solve(input), 165);
    }
}

common::read_main!();
