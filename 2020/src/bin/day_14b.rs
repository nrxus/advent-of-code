use regex::Regex;
use std::collections::HashMap;

fn solve(program: &str) -> u64 {
    let mask_regex = Regex::new(r"mask = (?P<mask>.*)").unwrap();
    let mem_regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();

    let mut masks = vec![[MaskBit::Keep; 36]];
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in program.trim().lines() {
        if let Some(captures) = mask_regex.captures(instruction) {
            masks = expand_mask(captures.name("mask").unwrap().as_str());
            continue;
        }
        let captures = mem_regex.captures(instruction).unwrap();
        let address: u16 = captures.name("address").unwrap().as_str().parse().unwrap();
        let value: u64 = captures.name("value").unwrap().as_str().parse().unwrap();
        let mut address = format!("{:036b}", address);

        masks.iter().for_each(|mask| {
            mask.iter()
                .enumerate()
                .filter_map(|(i, m)| match m {
                    MaskBit::Keep => None,
                    MaskBit::Set(b) => Some((i, *b)),
                })
                .for_each(|(i, b)| unsafe { address.as_mut_vec()[i] = b });

            memory.insert(u64::from_str_radix(&address, 2).unwrap(), value);
        });
    }

    memory.values().sum()
}

#[derive(Clone, Copy, Debug)]
enum MaskBit {
    Keep,
    Set(u8),
}

fn expand_mask(raw: &str) -> Vec<[MaskBit; 36]> {
    // pre-create the masks to do only one large allocation
    let num_floating = raw.chars().filter(|x| *x == 'X').count();
    let possibilities = 2_usize.pow(num_floating as u32);
    let mut masks = vec![[MaskBit::Keep; 36]; possibilities];

    recursively_mask(raw.as_bytes(), &mut masks, 0);

    masks
}

fn recursively_mask(raw: &[u8], masks: &mut [[MaskBit; 36]], index: usize) {
    for (i, c) in raw[index..].iter().enumerate() {
        match c {
            b'0' => {}
            b'1' => masks
                .iter_mut()
                .for_each(|m| m[index + i] = MaskBit::Set(b'1')),
            b'X' => {
                let (first, second) = masks.split_at_mut(masks.len() / 2);

                first
                    .iter_mut()
                    .for_each(|m| m[index + i] = MaskBit::Set(b'0'));
                recursively_mask(raw, first, index + i + 1);

                second
                    .iter_mut()
                    .for_each(|m| m[index + i] = MaskBit::Set(b'1'));
                recursively_mask(raw, second, index + i + 1);

                break;
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(solve(input), 208);
    }
}

common::read_main!();
